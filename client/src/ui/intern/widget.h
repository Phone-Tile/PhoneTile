//
//  widget.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef widget_h
#define widget_h

#include "event.h"

enum {
    UI_BUTTON
};

typedef struct UIwidget {
    char            type;
    struct UIwidget *prev, *next;
    
    UIevent *   event;
    Rectangle   location;
} UIwidget;

UIwidget *UIcreateButtonWidget(Rectangle rec, void (*callback)(void));
void UIrenderWidgets(void);
void UIaddWidget(UIwidget *w);
void UIremoveWidget(UIwidget *w);

#endif /* widget_h */
