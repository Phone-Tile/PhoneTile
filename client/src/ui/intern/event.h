//
//  event.h
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#ifndef event_h
#define event_h

#include "raylib.h"

enum {
    UI_LOCAL_CLICK,
    UI_GLOBAL_CLICK
};

typedef struct UIevent {
    char                type;
    struct UIevent      *prev, *next;
    
    Rectangle   location;
    void        (*callback)(void);
} UIevent;

void UIdestroyEvent(UIevent *e);
void UIaddEvent(UIevent *e);
void UIupdateEvents(void);

UIevent *UIcreateLocalClickEvent(Rectangle rec, void (*callback)(void));

#endif /* event_h */
