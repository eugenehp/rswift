import Foundation
#if canImport(CreateML) && os(macOS)
import CreateML
import CoreML
import TabularData
import Combine
import NaturalLanguage

@_cdecl("createml_available")
public func createmlAvailable() -> Bool { true }

// ══════════════════════════════════════════════════════════════════════════
// Common types
// ══════════════════════════════════════════════════════════════════════════

public typealias CB = @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void
public typealias ProgressCB = @convention(c) (Double, Int, Int, UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void

private func ms(_ p: UnsafePointer<UInt8>, _ l: Int) -> String {
    String(bytes: UnsafeBufferPointer(start: p, count: l), encoding: .utf8) ?? ""
}
private func ws(_ s: String, _ b: UnsafeMutablePointer<UInt8>, _ bl: Int) -> Int {
    let d = Array(s.utf8); let l = min(d.count, bl); for i in 0..<l { b[i] = d[i] }; return l
}
private func reply(_ cb: CB, _ ud: UnsafeMutableRawPointer?, _ msg: String, _ ok: Bool) {
    msg.withCString { cb(UnsafePointer(OpaquePointer($0)), msg.utf8.count, ok, ud) }
}
private func esc(_ s: String) -> String { s.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\"") }

private func parseJSON(_ p: UnsafePointer<UInt8>, _ l: Int) -> [String: Any]? {
    guard let data = ms(p,l).data(using: .utf8) else { return nil }
    return try? JSONSerialization.jsonObject(with: data) as? [String: Any]
}

private func cres(_ t: MLClassifierMetrics, _ v: MLClassifierMetrics, _ saved: Bool) -> String {
    "{\"training_error\":\(t.classificationError),\"validation_error\":\(v.classificationError),\"training_valid\":\(t.isValid),\"validation_valid\":\(v.isValid),\"model_saved\":\(saved)}"
}
private func rres(_ t: MLRegressorMetrics, _ v: MLRegressorMetrics, _ saved: Bool) -> String {
    "{\"training_max_error\":\(t.maximumError),\"training_rmse\":\(t.rootMeanSquaredError),\"validation_max_error\":\(v.maximumError),\"validation_rmse\":\(v.rootMeanSquaredError),\"model_saved\":\(saved)}"
}

// ══════════════════════════════════════════════════════════════════════════
// Unified train endpoint — JSON params, all model types
// ══════════════════════════════════════════════════════════════════════════
// 
// params JSON schema:
// {
//   "model_type": "text_classifier" | "image_classifier" | "sound_classifier" |
//                 "hand_pose_classifier" | "hand_action_classifier" | "style_transfer" |
//                 "boosted_tree_classifier" | "decision_tree_classifier" |
//                 "random_forest_classifier" | "logistic_regression_classifier" |
//                 "boosted_tree_regressor" | "decision_tree_regressor" |
//                 "random_forest_regressor" | "linear_regressor",
//   "training_data": "/path/to/dir_or_csv",
//   "output_path": "/path/to/model.mlmodel" (optional),
//   "target_column": "label" (for tabular),
//   "feature_columns": ["col1","col2"] (optional, for tabular),
//   "text_data": {"label":["text1"]} (for text classifier),
//   "style_image": "/path" (for style transfer),
//   "content_dir": "/path" (for style transfer),
//   
//   // Parameters (all optional, defaults used if omitted):
//   "max_iterations": 100,
//   "max_depth": 6,
//   "min_loss_reduction": 0.0,
//   "min_child_weight": 1.0,
//   "step_size": 0.3,
//   "early_stopping_rounds": null,
//   "row_subsample": 1.0,
//   "column_subsample": 1.0,
//   "random_seed": 42,
//   "l1_penalty": 0.0,
//   "l2_penalty": 0.01,
//   "convergence_threshold": 0.01,
//   "feature_rescaling": true,
//   "overlap_factor": 0.5,
//   "textel_density": 64,
//   "style_strength": 5,
//   "validation_ratio": 0.2,
//   "augmentation": ["rotation","blur","crop","flip","noise"],
//   
//   // Metadata (written into saved model):
//   "author": "...",
//   "description": "...",
//   "version": "1.0"
// }

@available(macOS 12.0, *)
@_cdecl("createml_train")
public func unifiedTrain(_ paramsJson: UnsafePointer<UInt8>, _ paramsLen: Int,
                         _ cb: CB, _ ud: UnsafeMutableRawPointer?) {
    guard let p = parseJSON(paramsJson, paramsLen) else { reply(cb,ud,"invalid params JSON",false); return }
    guard let modelType = p["model_type"] as? String else { reply(cb,ud,"missing model_type",false); return }
    
    let outPath = p["output_path"] as? String
    let metadata: MLModelMetadata? = {
        let author = p["author"] as? String ?? ""
        let desc = p["description"] as? String ?? ""
        let ver = p["version"] as? String ?? "1"
        if author.isEmpty && desc.isEmpty { return nil }
        return MLModelMetadata(author: author, shortDescription: desc, version: ver)
    }()
    
    do {
        switch modelType {
        case "text_classifier":
            guard let textData = p["text_data"] as? [String:[String]] else { reply(cb,ud,"missing text_data",false); return }
            var mp = MLTextClassifier.ModelParameters()
            if let lang = p["language"] as? String { mp.language = NLLanguage(rawValue: lang) }
            let c = try MLTextClassifier(trainingData: textData, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "image_classifier":
            guard let dir = p["training_data"] as? String else { reply(cb,ud,"missing training_data",false); return }
            let ds = MLImageClassifier.DataSource.labeledDirectories(at: URL(fileURLWithPath: dir))
            var mp = MLImageClassifier.ModelParameters()
            if let mi = p["max_iterations"] as? Int { mp.maxIterations = mi }
            if let augs = p["augmentation"] as? [String] {
                var opts: [MLImageClassifier.ImageAugmentationOptions] = []
                for a in augs {
                    switch a {
                    case "rotation": opts.append(.rotation)
                    case "blur": opts.append(.blur)
                    case "crop": opts.append(.crop)
                    case "flip": opts.append(MLImageClassifier.ImageAugmentationOptions(rawValue: 1))
                    case "noise": opts.append(.noise)
                    case "exposure": opts.append(.exposure)
                    default: break
                    }
                }
                mp.augmentationOptions = opts.reduce(MLImageClassifier.ImageAugmentationOptions()) { $0.union($1) }
            }
            let c = try MLImageClassifier(trainingData: ds, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "sound_classifier":
            guard let dir = p["training_data"] as? String else { reply(cb,ud,"missing training_data",false); return }
            var mp = MLSoundClassifier.ModelParameters()
            if let mi = p["max_iterations"] as? Int { mp.maxIterations = mi }
            if let of = p["overlap_factor"] as? Double { mp.overlapFactor = of }
            let c = try MLSoundClassifier(trainingData: .labeledDirectories(at: URL(fileURLWithPath: dir)), parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "hand_pose_classifier":
            guard let dir = p["training_data"] as? String else { reply(cb,ud,"missing training_data",false); return }
            let c = try MLHandPoseClassifier(trainingData: .labeledDirectories(at: URL(fileURLWithPath: dir)))
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "hand_action_classifier":
            guard let dir = p["training_data"] as? String else { reply(cb,ud,"missing training_data",false); return }
            let c = try MLHandActionClassifier(trainingData: .labeledDirectories(at: URL(fileURLWithPath: dir)))
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "style_transfer":
            guard let si = p["style_image"] as? String, let cd = p["content_dir"] as? String else {
                reply(cb,ud,"missing style_image or content_dir",false); return
            }
            var mp = MLStyleTransfer.ModelParameters()
            if let mi = p["max_iterations"] as? Int { mp.maxIterations = mi }
            if let td = p["textel_density"] as? Int { mp.textelDensity = td }
            if let ss = p["style_strength"] as? Int { mp.styleStrength = ss }
            let c = try MLStyleTransfer(trainingData: .images(styleImage: URL(fileURLWithPath: si), contentDirectory: URL(fileURLWithPath: cd)), parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,"{\"model_saved\":\(outPath != nil)}",true)
            
        case "boosted_tree_classifier":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLBoostedTreeClassifier.ModelParameters()
            applyTreeParamsCopy(p, &mp)
            let c = try MLBoostedTreeClassifier(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "decision_tree_classifier":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLDecisionTreeClassifier.ModelParameters()
            if let md = p["max_depth"] as? Int { mp.maxDepth = md }
            let c = try MLDecisionTreeClassifier(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "random_forest_classifier":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLRandomForestClassifier.ModelParameters()
            if let md = p["max_depth"] as? Int { mp.maxDepth = md }
            let c = try MLRandomForestClassifier(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "logistic_regression_classifier":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLLogisticRegressionClassifier.ModelParameters()
            if let mi = p["max_iterations"] as? Int { mp.maxIterations = mi }
            let c = try MLLogisticRegressionClassifier(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try c.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,cres(c.trainingMetrics,c.validationMetrics,outPath != nil),true)
            
        case "boosted_tree_regressor":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLBoostedTreeRegressor.ModelParameters()
            applyTreeParamsCopy(p, &mp)
            let r = try MLBoostedTreeRegressor(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try r.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,rres(r.trainingMetrics,r.validationMetrics,outPath != nil),true)
            
        case "decision_tree_regressor":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLDecisionTreeRegressor.ModelParameters()
            if let md = p["max_depth"] as? Int { mp.maxDepth = md }
            let r = try MLDecisionTreeRegressor(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try r.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,rres(r.trainingMetrics,r.validationMetrics,outPath != nil),true)
            
        case "random_forest_regressor":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLRandomForestRegressor.ModelParameters()
            if let md = p["max_depth"] as? Int { mp.maxDepth = md }
            let r = try MLRandomForestRegressor(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try r.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,rres(r.trainingMetrics,r.validationMetrics,outPath != nil),true)
            
        case "linear_regressor":
            let (df,tc,fc) = try loadTabular(p)
            var mp = MLLinearRegressor.ModelParameters()
            if let mi = p["max_iterations"] as? Int { mp.maxIterations = mi }
            if let l1 = p["l1_penalty"] as? Double { mp.l1Penalty = l1 }
            if let l2 = p["l2_penalty"] as? Double { mp.l2Penalty = l2 }
            if let ss = p["step_size"] as? Double { mp.stepSize = ss }
            if let ct = p["convergence_threshold"] as? Double { mp.convergenceThreshold = ct }
            if let fr = p["feature_rescaling"] as? Bool { mp.featureRescaling = fr }
            let r = try MLLinearRegressor(trainingData: df, targetColumn: tc, featureColumns: fc, parameters: mp)
            if let o = outPath { try r.write(to: URL(fileURLWithPath: o), metadata: metadata) }
            reply(cb,ud,rres(r.trainingMetrics,r.validationMetrics,outPath != nil),true)
            
        default:
            reply(cb,ud,"unknown model_type: \(modelType)",false)
        }
    } catch {
        reply(cb,ud,"\(error.localizedDescription)",false)
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Predict — unified
// ══════════════════════════════════════════════════════════════════════════

@available(macOS 12.0, *)
@_cdecl("createml_predict")
public func unifiedPredict(_ paramsJson: UnsafePointer<UInt8>, _ paramsLen: Int,
                           _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let p = parseJSON(paramsJson, paramsLen),
          let modelPath = p["model_path"] as? String else { return -1 }
    guard let model = try? MLModel(contentsOf: URL(fileURLWithPath: modelPath)) else { return -1 }
    
    let predName = model.modelDescription.predictedFeatureName ?? "label"
    
    if let text = p["text"] as? String {
        // Text prediction
        guard let prov = try? MLDictionaryFeatureProvider(dictionary: ["text": MLFeatureValue(string: text)]),
              let result = try? model.prediction(from: prov),
              let label = result.featureValue(for: predName)?.stringValue
        else { return -1 }
        return ws(label, buf, bufLen)
    }
    
    if let csvPath = p["csv_path"] as? String {
        // Tabular batch prediction
        guard let df = try? DataFrame(contentsOfCSVFile: URL(fileURLWithPath: csvPath)) else { return -1 }
        var preds: [String] = []
        for row in df.rows.prefix(10000) {
            var dict: [String: MLFeatureValue] = [:]
            for col in df.columns {
                let v: Any? = row[col.name]
                if let s = v as? String { dict[col.name] = MLFeatureValue(string: s) }
                else if let d = v as? Double { dict[col.name] = MLFeatureValue(double: d) }
                else if let i = v as? Int { dict[col.name] = MLFeatureValue(int64: Int64(i)) }
            }
            if let prov = try? MLDictionaryFeatureProvider(dictionary: dict),
               let r = try? model.prediction(from: prov), let val = r.featureValue(for: predName) {
                if let s = val.stringValue as String? { preds.append("\"\(esc(s))\"") }
                else if val.type == .double { preds.append("\(val.doubleValue)") }
                else if val.type == .int64 { preds.append("\(val.int64Value)") }
                else { preds.append("\"\(val)\"") }
            }
        }
        return ws("[" + preds.joined(separator: ",") + "]", buf, bufLen)
    }
    
    return -1
}

// ══════════════════════════════════════════════════════════════════════════
// Evaluate — unified
// ══════════════════════════════════════════════════════════════════════════

@available(macOS 12.0, *)
@_cdecl("createml_evaluate")
public func unifiedEval(_ paramsJson: UnsafePointer<UInt8>, _ paramsLen: Int,
                        _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    guard let p = parseJSON(paramsJson, paramsLen),
          let modelPath = p["model_path"] as? String,
          let csvPath = p["csv_path"] as? String,
          let targetCol = p["target_column"] as? String
    else { return -1 }
    
    guard let model = try? MLModel(contentsOf: URL(fileURLWithPath: modelPath)),
          let df = try? DataFrame(contentsOfCSVFile: URL(fileURLWithPath: csvPath))
    else { return -1 }
    
    let predName = model.modelDescription.predictedFeatureName ?? "label"
    var correct = 0, total = 0, sumSquaredError = 0.0
    var isRegressor = false
    
    for row in df.rows {
        guard let label = row[targetCol] else { continue }
        total += 1
        var dict: [String: MLFeatureValue] = [:]
        for col in df.columns where col.name != targetCol {
            let v: Any? = row[col.name]
            if let s = v as? String { dict[col.name] = MLFeatureValue(string: s) }
            else if let d = v as? Double { dict[col.name] = MLFeatureValue(double: d) }
            else if let i = v as? Int { dict[col.name] = MLFeatureValue(int64: Int64(i)) }
        }
        guard let prov = try? MLDictionaryFeatureProvider(dictionary: dict),
              let r = try? model.prediction(from: prov),
              let pv = r.featureValue(for: predName) else { continue }
        
        if pv.type == .double || pv.type == .int64 {
            isRegressor = true
            let predicted = pv.type == .double ? pv.doubleValue : Double(pv.int64Value)
            let actual: Double
            if let d = label as? Double { actual = d }
            else if let i = label as? Int { actual = Double(i) }
            else { continue }
            sumSquaredError += (predicted - actual) * (predicted - actual)
        } else {
            if "\(pv)" == "\(label)" { correct += 1 }
        }
    }
    
    if isRegressor {
        let rmse = total > 0 ? (sumSquaredError / Double(total)).squareRoot() : 0
        return ws("{\"rmse\":\(rmse),\"total\":\(total),\"type\":\"regressor\"}", buf, bufLen)
    } else {
        let acc = total > 0 ? Double(correct)/Double(total) : 0
        return ws("{\"accuracy\":\(acc),\"correct\":\(correct),\"total\":\(total),\"type\":\"classifier\"}", buf, bufLen)
    }
}

// ══════════════════════════════════════════════════════════════════════════
// Model info
// ══════════════════════════════════════════════════════════════════════════

@_cdecl("createml_model_info")
public func modelInfo(_ p: UnsafePointer<UInt8>, _ pl: Int, _ b: UnsafeMutablePointer<UInt8>, _ bl: Int) -> Int {
    guard let m = try? MLModel(contentsOf: URL(fileURLWithPath: ms(p,pl))) else { return -1 }
    let d = m.modelDescription
    let ins = d.inputDescriptionsByName.map { "{\"name\":\"\(esc($0.key))\",\"type\":\($0.value.type.rawValue)}" }.joined(separator: ",")
    let outs = d.outputDescriptionsByName.map { "{\"name\":\"\(esc($0.key))\",\"type\":\($0.value.type.rawValue)}" }.joined(separator: ",")
    let pn = d.predictedFeatureName ?? ""
    return ws("{\"inputs\":[\(ins)],\"outputs\":[\(outs)],\"predicted_feature\":\"\(pn)\"}", b, bl)
}

// ══════════════════════════════════════════════════════════════════════════
// DataTable ops
// ══════════════════════════════════════════════════════════════════════════

@_cdecl("createml_dt_info")
public func dtInfo(_ p: UnsafePointer<UInt8>, _ pl: Int, _ b: UnsafeMutablePointer<UInt8>, _ bl: Int) -> Int {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(p,pl))) else { return -1 }
    let cols = t.columnTypes.map { "{\"name\":\"\(esc($0.key))\",\"type\":\"\($0.value)\"}" }.joined(separator: ",")
    return ws("{\"rows\":\(t.size.rows),\"columns\":\(t.size.columns),\"column_info\":[\(cols)]}", b, bl)
}

@_cdecl("createml_dt_write_csv")
public func dtWriteCSV(_ i: UnsafePointer<UInt8>, _ il: Int, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    (try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))).writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

@_cdecl("createml_dt_split")
public func dtSplit(_ p: UnsafePointer<UInt8>, _ pl: Int, _ ratio: Double, _ seed: Int,
                    _ to: UnsafePointer<UInt8>, _ tol: Int, _ teo: UnsafePointer<UInt8>, _ teol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(p,pl))) else { return false }
    let (train,test) = t.randomSplit(by: ratio, seed: seed)
    return (try? train.writeCSV(to: URL(fileURLWithPath: ms(to,tol)))) != nil &&
           (try? test.writeCSV(to: URL(fileURLWithPath: ms(teo,teol)))) != nil
}

@_cdecl("createml_dt_drop_missing")
public func dtDropMissing(_ i: UnsafePointer<UInt8>, _ il: Int, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))) else { return false }
    return (try? t.dropMissing().writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

@_cdecl("createml_dt_drop_duplicates")
public func dtDropDups(_ i: UnsafePointer<UInt8>, _ il: Int, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))) else { return false }
    return (try? t.dropDuplicates().writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

@_cdecl("createml_dt_sort")
public func dtSort(_ i: UnsafePointer<UInt8>, _ il: Int, _ c: UnsafePointer<UInt8>, _ cl: Int,
                   _ asc: Bool, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))) else { return false }
    return (try? t.sort(columnNamed: ms(c,cl), byIncreasingOrder: asc).writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

@_cdecl("createml_dt_prefix")
public func dtPrefix(_ i: UnsafePointer<UInt8>, _ il: Int, _ n: Int, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))) else { return false }
    return (try? t.prefix(n).writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

@_cdecl("createml_dt_suffix")
public func dtSuffix(_ i: UnsafePointer<UInt8>, _ il: Int, _ n: Int, _ o: UnsafePointer<UInt8>, _ ol: Int) -> Bool {
    guard let t = try? MLDataTable(contentsOf: URL(fileURLWithPath: ms(i,il))) else { return false }
    return (try? t.suffix(n).writeCSV(to: URL(fileURLWithPath: ms(o,ol)))) != nil
}

// ══════════════════════════════════════════════════════════════════════════
// Helpers
// ══════════════════════════════════════════════════════════════════════════

@available(macOS 12.0, *)
private func loadTabular(_ p: [String: Any]) throws -> (DataFrame, String, [String]?) {
    guard let csv = p["training_data"] as? String else { throw NSError(domain: "CreateML", code: 1, userInfo: [NSLocalizedDescriptionKey: "missing training_data"]) }
    guard let target = p["target_column"] as? String else { throw NSError(domain: "CreateML", code: 2, userInfo: [NSLocalizedDescriptionKey: "missing target_column"]) }
    let features = p["feature_columns"] as? [String]
    let df = try DataFrame(contentsOfCSVFile: URL(fileURLWithPath: csv))
    return (df, target, features)
}

// Overloads avoid Swift exclusivity violations from passing multiple inout fields
// of the same struct simultaneously.
private func applyTreeParamsCopy(_ p: [String: Any], _ mp: inout MLBoostedTreeClassifier.ModelParameters) {
    if let v = p["max_iterations"] as? Int     { mp.maxIterations = v }
    if let v = p["max_depth"] as? Int          { mp.maxDepth = v }
    if let v = p["min_loss_reduction"] as? Double { mp.minLossReduction = v }
    if let v = p["min_child_weight"] as? Double { mp.minChildWeight = v }
    if let v = p["step_size"] as? Double       { mp.stepSize = v }
    if let v = p["random_seed"] as? Int        { mp.randomSeed = v }
    if let v = p["row_subsample"] as? Double   { mp.rowSubsample = v }
    if let v = p["column_subsample"] as? Double { mp.columnSubsample = v }
    if let v = p["early_stopping_rounds"] as? Int { mp.earlyStoppingRounds = v }
}
private func applyTreeParamsCopy(_ p: [String: Any], _ mp: inout MLBoostedTreeRegressor.ModelParameters) {
    if let v = p["max_iterations"] as? Int     { mp.maxIterations = v }
    if let v = p["max_depth"] as? Int          { mp.maxDepth = v }
    if let v = p["min_loss_reduction"] as? Double { mp.minLossReduction = v }
    if let v = p["min_child_weight"] as? Double { mp.minChildWeight = v }
    if let v = p["step_size"] as? Double       { mp.stepSize = v }
    if let v = p["random_seed"] as? Int        { mp.randomSeed = v }
    if let v = p["row_subsample"] as? Double   { mp.rowSubsample = v }
    if let v = p["column_subsample"] as? Double { mp.columnSubsample = v }
    if let v = p["early_stopping_rounds"] as? Int { mp.earlyStoppingRounds = v }
}

#else
@_cdecl("createml_available") public func createmlAvailable() -> Bool { false }
#endif
