import Foundation
import WeatherKit
import CoreLocation

@_cdecl("weatherkit_available")
public func weatherkitAvailable() -> Bool { true }

// Attribution (async in newer SDKs)
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_attribution_legal_page_url")
public func weatherkitAttributionLegalPageURL(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    Task {
        let attr = try? await WeatherService.shared.attribution
        let url = attr?.legalPageURL.absoluteString ?? ""
        url.withCString { ptr in
            cb(UnsafePointer(OpaquePointer(ptr)), url.utf8.count, ud)
        }
    }
}

@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_attribution_mark_url")
public func weatherkitAttributionMarkURL(
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    Task {
        let attr = try? await WeatherService.shared.attribution
        let url = attr?.combinedMarkDarkURL.absoluteString ?? ""
        url.withCString { ptr in
            cb(UnsafePointer(OpaquePointer(ptr)), url.utf8.count, ud)
        }
    }
}

// Current weather
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_fetch_current")
public func weatherkitFetchCurrent(
    _ lat: Double, _ lon: Double,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    Task {
        do {
            let weather = try await WeatherService.shared.weather(for: location)
            let c = weather.currentWeather
            let result = """
            {"temperature":\(c.temperature.converted(to: .celsius).value),\
            "apparentTemperature":\(c.apparentTemperature.converted(to: .celsius).value),\
            "humidity":\(c.humidity),\
            "dewPoint":\(c.dewPoint.converted(to: .celsius).value),\
            "windSpeed":\(c.wind.speed.converted(to: .metersPerSecond).value),\
            "windGust":\(c.wind.gust?.converted(to: .metersPerSecond).value ?? 0),\
            "windDirection":\(c.wind.direction.converted(to: .degrees).value),\
            "pressure":\(c.pressure.converted(to: .hectopascals).value),\
            "pressureTrend":"\(c.pressureTrend.rawValue)",\
            "visibility":\(c.visibility.converted(to: .meters).value),\
            "uvIndex":\(c.uvIndex.value),\
            "cloudCover":\(c.cloudCover),\
            "condition":"\(c.condition.rawValue)",\
            "conditionDescription":"\(c.condition.description)",\
            "symbolName":"\(c.symbolName)",\
            "isDaylight":\(c.isDaylight)}
            """
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            let err = "error:\(error.localizedDescription)"
            err.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), err.utf8.count, false, ud)
            }
        }
    }
}

// Hourly forecast
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_fetch_hourly")
public func weatherkitFetchHourly(
    _ lat: Double, _ lon: Double, _ hours: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    Task {
        do {
            let weather = try await WeatherService.shared.weather(for: location)
            var items: [String] = []
            for hour in weather.hourlyForecast.prefix(hours) {
                let iso = ISO8601DateFormatter().string(from: hour.date)
                items.append("""
                {"date":"\(iso)",\
                "temperature":\(hour.temperature.converted(to: .celsius).value),\
                "humidity":\(hour.humidity),\
                "windSpeed":\(hour.wind.speed.converted(to: .metersPerSecond).value),\
                "precipitationChance":\(hour.precipitationChance),\
                "condition":"\(hour.condition.rawValue)",\
                "uvIndex":\(hour.uvIndex.value),\
                "isDaylight":\(hour.isDaylight)}
                """)
            }
            let result = "[" + items.joined(separator: ",") + "]"
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

// Daily forecast
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_fetch_daily")
public func weatherkitFetchDaily(
    _ lat: Double, _ lon: Double, _ days: Int,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    Task {
        do {
            let weather = try await WeatherService.shared.weather(for: location)
            var items: [String] = []
            for day in weather.dailyForecast.prefix(days) {
                let iso = ISO8601DateFormatter().string(from: day.date)
                items.append("""
                {"date":"\(iso)",\
                "highTemperature":\(day.highTemperature.converted(to: .celsius).value),\
                "lowTemperature":\(day.lowTemperature.converted(to: .celsius).value),\
                "precipitationChance":\(day.precipitationChance),\
                "precipitationAmount":\(day.precipitationAmountByType.mixed.converted(to: .millimeters).value),\
                "snowfallAmount":\(day.precipitationAmountByType.mixed.converted(to: .centimeters).value),\
                "condition":"\(day.condition.rawValue)",\
                "conditionDescription":"\(day.condition.description)",\
                "uvIndexMax":\(day.uvIndex.value),\
                "windSpeedMax":\(day.wind.speed.converted(to: .metersPerSecond).value),\
                "moonPhase":"\(day.moon.phase.rawValue)",\
                "sunriseDate":"\(day.sun.sunrise.map { ISO8601DateFormatter().string(from: $0) } ?? "")",\
                "sunsetDate":"\(day.sun.sunset.map { ISO8601DateFormatter().string(from: $0) } ?? "")"}
                """)
            }
            let result = "[" + items.joined(separator: ",") + "]"
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

// Weather alerts
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_fetch_alerts")
public func weatherkitFetchAlerts(
    _ lat: Double, _ lon: Double,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    Task {
        do {
            let weather = try await WeatherService.shared.weather(for: location, including: .alerts)
            var items: [String] = []
            if let alerts = weather {
                for alert in alerts {
                    items.append("""
                    {"summary":"\(esc(alert.summary))",\
                    "severity":"\(alert.severity.rawValue)",\
                    "source":"\(esc(alert.source))",\
                    "region":"\(esc(alert.region ?? ""))"}
                    """)
                }
            }
            let result = "[" + items.joined(separator: ",") + "]"
            result.withCString { ptr in
                cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
            }
        } catch {
            cb(UnsafePointer(OpaquePointer(bitPattern: 1))!, 0, false, ud)
        }
    }
}

// Availability check per location
@available(macOS 13.0, iOS 16.0, *)
@_cdecl("weatherkit_check_availability")
public func weatherkitCheckAvailability(
    _ lat: Double, _ lon: Double,
    _ cb: @convention(c) (UnsafePointer<UInt8>, Int, Bool, UnsafeMutableRawPointer?) -> Void,
    _ ud: UnsafeMutableRawPointer?
) {
    let location = CLLocation(latitude: lat, longitude: lon)
    Task {
        // WeatherService.availability was removed; infer from weather() success
        var avail: [String] = []
        if let _ = try? await WeatherService.shared.weather(for: location) {
            avail.append("current"); avail.append("hourly"); avail.append("daily")
            avail.append("minute"); avail.append("alerts")
        }
        let result = "[\"" + avail.joined(separator: "\",\"") + "\"]"
        result.withCString { ptr in
            cb(UnsafePointer(OpaquePointer(ptr)), result.utf8.count, true, ud)
        }
    }
}

private func esc(_ s: String) -> String {
    s.replacingOccurrences(of: "\\", with: "\\\\")
     .replacingOccurrences(of: "\"", with: "\\\"")
     .replacingOccurrences(of: "\n", with: "\\n")
}

private func writeStr(_ s: String, _ buf: UnsafeMutablePointer<UInt8>, _ bufLen: Int) -> Int {
    let data = Array(s.utf8)
    let len = min(data.count, bufLen)
    for i in 0..<len { buf[i] = data[i] }
    return len
}
