//! ObjC selector constants for EventKit.
#![allow(dead_code)]

// ── EKEventStore (31 methods, 3 properties) ──
pub mod e_k_event_store {
    pub const CLASS: &[u8] = b"EKEventStore\0";
    pub const SEL_EVENT_STORE_IDENTIFIER: &[u8] = b"eventStoreIdentifier\0";
    pub const SEL_SET_EVENT_STORE_IDENTIFIER: &[u8] = b"setEventStoreIdentifier:\0";
    pub const SEL_ENDIF: &[u8] = b"endif\0";
    pub const SEL_SET_ENDIF: &[u8] = b"setEndif:\0";
    pub const SEL_DEFAULT_CALENDAR_FOR_NEW_EVENTS: &[u8] = b"defaultCalendarForNewEvents\0";
    pub const SEL_SET_DEFAULT_CALENDAR_FOR_NEW_EVENTS: &[u8] = b"setDefaultCalendarForNewEvents:\0";
    pub const SEL_AUTHORIZATION_STATUS_FOR_ENTITY_TYPE: &[u8] = b"authorizationStatusForEntityType:\0";
    pub const SEL_REQUEST_FULL_ACCESS_TO_EVENTS_WITH_COMPLETION: &[u8] = b"requestFullAccessToEventsWithCompletion:\0";
    pub const SEL_REQUEST_WRITE_ONLY_ACCESS_TO_EVENTS_WITH_COMPLETION: &[u8] = b"requestWriteOnlyAccessToEventsWithCompletion:\0";
    pub const SEL_REQUEST_FULL_ACCESS_TO_REMINDERS_WITH_COMPLETION: &[u8] = b"requestFullAccessToRemindersWithCompletion:\0";
    pub const SEL_REQUEST_ACCESS_TO_ENTITY_TYPE: &[u8] = b"requestAccessToEntityType:completion:\0";
    pub const SEL_SOURCE_WITH_IDENTIFIER: &[u8] = b"sourceWithIdentifier:\0";
    pub const SEL_CALENDARS_FOR_ENTITY_TYPE: &[u8] = b"calendarsForEntityType:\0";
    pub const SEL_DEFAULT_CALENDAR_FOR_NEW_REMINDERS: &[u8] = b"defaultCalendarForNewReminders\0";
    pub const SEL_CALENDAR_WITH_IDENTIFIER: &[u8] = b"calendarWithIdentifier:\0";
    pub const SEL_SAVE_CALENDAR: &[u8] = b"saveCalendar:commit:error:\0";
    pub const SEL_REMOVE_CALENDAR: &[u8] = b"removeCalendar:commit:error:\0";
    pub const SEL_CALENDAR_ITEM_WITH_IDENTIFIER: &[u8] = b"calendarItemWithIdentifier:\0";
    pub const SEL_CALENDAR_ITEMS_WITH_EXTERNAL_IDENTIFIER: &[u8] = b"calendarItemsWithExternalIdentifier:\0";
    pub const SEL_SAVE_EVENT: &[u8] = b"saveEvent:span:error:\0";
    pub const SEL_REMOVE_EVENT: &[u8] = b"removeEvent:span:error:\0";
    pub const SEL_EVENT_WITH_IDENTIFIER: &[u8] = b"eventWithIdentifier:\0";
    pub const SEL_EVENTS_MATCHING_PREDICATE: &[u8] = b"eventsMatchingPredicate:\0";
    pub const SEL_ENUMERATE_EVENTS_MATCHING_PREDICATE: &[u8] = b"enumerateEventsMatchingPredicate:usingBlock:\0";
    pub const SEL_PREDICATE_FOR_EVENTS_WITH_START_DATE: &[u8] = b"predicateForEventsWithStartDate:endDate:calendars:\0";
    pub const SEL_SAVE_REMINDER: &[u8] = b"saveReminder:commit:error:\0";
    pub const SEL_REMOVE_REMINDER: &[u8] = b"removeReminder:commit:error:\0";
    pub const SEL_CANCEL_FETCH_REQUEST: &[u8] = b"cancelFetchRequest:\0";
    pub const SEL_PREDICATE_FOR_REMINDERS_IN_CALENDARS: &[u8] = b"predicateForRemindersInCalendars:\0";
    pub const SEL_PREDICATE_FOR_INCOMPLETE_REMINDERS_WITH_DUE_DATE_STARTING: &[u8] = b"predicateForIncompleteRemindersWithDueDateStarting:ending:calendars:\0";
    pub const SEL_PREDICATE_FOR_COMPLETED_REMINDERS_WITH_COMPLETION_DATE_STARTING: &[u8] = b"predicateForCompletedRemindersWithCompletionDateStarting:ending:calendars:\0";
    pub const SEL_COMMIT: &[u8] = b"commit:\0";
    pub const SEL_RESET: &[u8] = b"reset\0";
    pub const SEL_REFRESH_SOURCES_IF_NECESSARY: &[u8] = b"refreshSourcesIfNecessary\0";
}

// ── EKEvent (3 methods, 9 properties) ──
pub mod e_k_event {
    pub const SEL_EVENT_IDENTIFIER: &[u8] = b"eventIdentifier\0";
    pub const SEL_SET_EVENT_IDENTIFIER: &[u8] = b"setEventIdentifier:\0";
    pub const SEL_ALL_DAY: &[u8] = b"allDay\0";
    pub const SEL_SET_ALL_DAY: &[u8] = b"setAllDay:\0";
    pub const SEL_START_DATE: &[u8] = b"startDate\0";
    pub const SEL_SET_START_DATE: &[u8] = b"setStartDate:\0";
    pub const SEL_END_DATE: &[u8] = b"endDate\0";
    pub const SEL_SET_END_DATE: &[u8] = b"setEndDate:\0";
    pub const SEL_ORGANIZER: &[u8] = b"organizer\0";
    pub const SEL_SET_ORGANIZER: &[u8] = b"setOrganizer:\0";
    pub const SEL_AVAILABILITY: &[u8] = b"availability\0";
    pub const SEL_SET_AVAILABILITY: &[u8] = b"setAvailability:\0";
    pub const SEL_STATUS: &[u8] = b"status\0";
    pub const SEL_SET_STATUS: &[u8] = b"setStatus:\0";
    pub const SEL_IS_DETACHED: &[u8] = b"isDetached\0";
    pub const SEL_SET_IS_DETACHED: &[u8] = b"setIsDetached:\0";
    pub const SEL_EVENT_WITH_EVENT_STORE: &[u8] = b"eventWithEventStore:\0";
    pub const SEL_COMPARE_START_DATE_WITH_EVENT: &[u8] = b"compareStartDateWithEvent:\0";
    pub const SEL_REFRESH: &[u8] = b"refresh\0";
}

// ── EKReminder (1 methods, 5 properties) ──
pub mod e_k_reminder {
    pub const SEL_START_DATE_COMPONENTS: &[u8] = b"startDateComponents\0";
    pub const SEL_SET_START_DATE_COMPONENTS: &[u8] = b"setStartDateComponents:\0";
    pub const SEL_DUE_DATE_COMPONENTS: &[u8] = b"dueDateComponents\0";
    pub const SEL_SET_DUE_DATE_COMPONENTS: &[u8] = b"setDueDateComponents:\0";
    pub const SEL_COMPLETED: &[u8] = b"completed\0";
    pub const SEL_SET_COMPLETED: &[u8] = b"setCompleted:\0";
    pub const SEL_COMPLETION_DATE: &[u8] = b"completionDate\0";
    pub const SEL_SET_COMPLETION_DATE: &[u8] = b"setCompletionDate:\0";
    pub const SEL_PRIORITY: &[u8] = b"priority\0";
    pub const SEL_SET_PRIORITY: &[u8] = b"setPriority:\0";
    pub const SEL_REMINDER_WITH_EVENT_STORE: &[u8] = b"reminderWithEventStore:\0";
}

// ── EKCalendar (2 methods, 5 properties) ──
pub mod e_k_calendar {
    pub const SEL_SOURCE: &[u8] = b"source\0";
    pub const SEL_SET_SOURCE: &[u8] = b"setSource:\0";
    pub const SEL_TITLE: &[u8] = b"title\0";
    pub const SEL_SET_TITLE: &[u8] = b"setTitle:\0";
    pub const SEL_TYPE: &[u8] = b"type\0";
    pub const SEL_SET_TYPE: &[u8] = b"setType:\0";
    pub const SEL_ALLOWS_CONTENT_MODIFICATIONS: &[u8] = b"allowsContentModifications\0";
    pub const SEL_SET_ALLOWS_CONTENT_MODIFICATIONS: &[u8] = b"setAllowsContentModifications:\0";
    pub const SEL_SUPPORTED_EVENT_AVAILABILITIES: &[u8] = b"supportedEventAvailabilities\0";
    pub const SEL_SET_SUPPORTED_EVENT_AVAILABILITIES: &[u8] = b"setSupportedEventAvailabilities:\0";
    pub const SEL_CALENDAR_WITH_EVENT_STORE: &[u8] = b"calendarWithEventStore:\0";
    pub const SEL_CALENDAR_FOR_ENTITY_TYPE: &[u8] = b"calendarForEntityType:eventStore:\0";
}

// ── EKAlarm (2 methods, 4 properties) ──
pub mod e_k_alarm {
    pub const SEL_RELATIVE_OFFSET: &[u8] = b"relativeOffset\0";
    pub const SEL_SET_RELATIVE_OFFSET: &[u8] = b"setRelativeOffset:\0";
    pub const SEL_ABSOLUTE_DATE: &[u8] = b"absoluteDate\0";
    pub const SEL_SET_ABSOLUTE_DATE: &[u8] = b"setAbsoluteDate:\0";
    pub const SEL_STRUCTURED_LOCATION: &[u8] = b"structuredLocation\0";
    pub const SEL_SET_STRUCTURED_LOCATION: &[u8] = b"setStructuredLocation:\0";
    pub const SEL_PROXIMITY: &[u8] = b"proximity\0";
    pub const SEL_SET_PROXIMITY: &[u8] = b"setProximity:\0";
    pub const SEL_ALARM_WITH_ABSOLUTE_DATE: &[u8] = b"alarmWithAbsoluteDate:\0";
    pub const SEL_ALARM_WITH_RELATIVE_OFFSET: &[u8] = b"alarmWithRelativeOffset:\0";
}

// ── EKRecurrenceRule (0 methods, 11 properties) ──
pub mod e_k_recurrence_rule {
    pub const SEL_CALENDAR_IDENTIFIER: &[u8] = b"calendarIdentifier\0";
    pub const SEL_SET_CALENDAR_IDENTIFIER: &[u8] = b"setCalendarIdentifier:\0";
    pub const SEL_RECURRENCE_END: &[u8] = b"recurrenceEnd\0";
    pub const SEL_SET_RECURRENCE_END: &[u8] = b"setRecurrenceEnd:\0";
    pub const SEL_FREQUENCY: &[u8] = b"frequency\0";
    pub const SEL_SET_FREQUENCY: &[u8] = b"setFrequency:\0";
    pub const SEL_INTERVAL: &[u8] = b"interval\0";
    pub const SEL_SET_INTERVAL: &[u8] = b"setInterval:\0";
    pub const SEL_FIRST_DAY_OF_THE_WEEK: &[u8] = b"firstDayOfTheWeek\0";
    pub const SEL_SET_FIRST_DAY_OF_THE_WEEK: &[u8] = b"setFirstDayOfTheWeek:\0";
    pub const SEL_DAYS_OF_THE_WEEK: &[u8] = b"daysOfTheWeek\0";
    pub const SEL_SET_DAYS_OF_THE_WEEK: &[u8] = b"setDaysOfTheWeek:\0";
    pub const SEL_DAYS_OF_THE_MONTH: &[u8] = b"daysOfTheMonth\0";
    pub const SEL_SET_DAYS_OF_THE_MONTH: &[u8] = b"setDaysOfTheMonth:\0";
    pub const SEL_DAYS_OF_THE_YEAR: &[u8] = b"daysOfTheYear\0";
    pub const SEL_SET_DAYS_OF_THE_YEAR: &[u8] = b"setDaysOfTheYear:\0";
    pub const SEL_WEEKS_OF_THE_YEAR: &[u8] = b"weeksOfTheYear\0";
    pub const SEL_SET_WEEKS_OF_THE_YEAR: &[u8] = b"setWeeksOfTheYear:\0";
    pub const SEL_MONTHS_OF_THE_YEAR: &[u8] = b"monthsOfTheYear\0";
    pub const SEL_SET_MONTHS_OF_THE_YEAR: &[u8] = b"setMonthsOfTheYear:\0";
    pub const SEL_SET_POSITIONS: &[u8] = b"setPositions\0";
    pub const SEL_SET_SET_POSITIONS: &[u8] = b"setSetPositions:\0";
}

// Total: 111 selector constants
