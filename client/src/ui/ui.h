//
//  ui.h
//
//  Created by Emmanuel Mera on 18/10/2023.
//

#ifndef ui_h
#define ui_h

/*
 I think the general design will work just fine, you can addapt it if you prefer
 YOU ARE COMPLETLY free to redo the entire desing of buttons etc .. if the behaviour / rendering doesn't feets you !
 */

enum {
    UI_LOCAL_CLICK,
    UI_GLOBAL_CLICK
};

typedef struct UIevent UIevent;

/**
 Destroy a UI event which has previously created by ANY event builder fonction
 */
extern void UIdestroyEvent(UIevent *e);

/**
 Add an event in the event list, it will then be active until it is removed from the event list
 */
extern void UIaddEvent(UIevent *e);

/**
 This function check all the activated events and trigger the one that needs to be trigger
 */
extern void UIupdateEvents(void);

/**
 This is an event builder fonction. It creates an event (not activated) which is triggered (call back is called)
 when there is a touch in the rectangle
 */
extern UIevent *UIcreateLocalClickEvent(Rectangle rec, void (*callback)(void));

enum {
    UI_BUTTON
};

typedef struct UIwidget UIwidget;

/**
 Create a button widget (rec is the size of the button, callback is the callback function).
 It is by default not activated, that is it is not drawn and the event associated is not activated.
 */
extern UIwidget *UIcreateButtonWidget(Rectangle rec, void (*callback)(void));

/**
 Render all the ativated widgets.
 */
extern void UIrenderWidgets(void);

/**
 Add the widget to the activated widget list. The widget become active (that is, it is being drawn and the event become activated)
 */
extern void UIaddWidget(UIwidget *w);

/**
 Remove the widget from the activated widget list. The widget become inactive.
 WARNING : the widget MUST BE actviated for this function to be used safely, if not it WILL crash.
 */
extern void UIremoveWidget(UIwidget *w);

#endif /* ui_h */
