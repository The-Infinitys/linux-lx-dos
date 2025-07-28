#pragma once

typedef enum AppEventType {
    AppEventType_None,
    AppEventType_TrayClicked,
    AppEventType_TrayDoubleClicked,
    AppEventType_MenuItemClicked
} AppEventType;

typedef enum QtWindowEventType {
    QtWindowEvent_None,
    QtWindowEvent_Closed
} QtWindowEventType;

typedef enum QtElementType {
    QtElementType_Button,
    QtElementType_Label,
    QtElementType_LineEdit,
    QtElementType_Widget
} QtElementType;

typedef enum QtElementEventType {
    QtElementEventType_None,
    QtElementEventType_Clicked,
    QtElementEventType_TextChanged,
    QtElementEventType_EditingFinished
} QtElementEventType;
