//
//  event.c
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#include <assert.h>

#include "event.h"
#include "MEM_alloc.h"

/* ---------------------------- */
/*  local variables             */
/* ---------------------------- */

static UIevent *events_head = NULL;

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static _Bool check_in_list(UIevent *e);

/* ---------------------------- */
/*  implementation              */
/* ---------------------------- */

void UIcreateEvent(void) {
    
}

void UIaddEvent(UIevent *e) {
    if (events_head) {
        e->next = events_head;
        e->prev = events_head->prev;
        
        events_head->prev->next = e;
        events_head->prev = e;
        events_head = e;
    } else {
        events_head = e;
        e->next = e;
        events_head->prev = e;
    }
}

void UIremoveEvent(UIevent *e) {
    assert(check_in_list(e));
    
    if (e->next == e) {
        events_head = NULL;
    }
    
    if (events_head == e) {
        events_head = e->next;
        e->next->prev = e->prev;
        e->prev->next = e->next;
    }
}

void UIdestroyEvent(UIevent *e) {
    if (check_in_list(e)) {
        UIremoveEvent(e);
    }
    
    MEM_free(e);
}

void UIupdateEvents(void) {
    UIevent *e = events_head;
    
    if (!e) return;
    
    do {
        switch (e->type) {
            case UI_GLOBAL_CLICK:
                if (IsMouseButtonDown(MOUSE_BUTTON_LEFT))
                    e->callback();
                break;
                
            case UI_LOCAL_CLICK:
                if (IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
                    int x = GetMouseX(), y = GetMouseY();
                    
                    if (CheckCollisionPointRec((Vector2){x, y}, e->location))
                        e->callback();
                }
                break;
                
            default:
                break;
        }
        
        e = e->next;
    } while (e != events_head);
}

void UIgetEventState(void) {
    
}

UIevent *UIcreateLocalClickEvent(Rectangle rec, void (*callback)(void)) {
    UIevent *e = MEM_calloc(sizeof(UIevent), __func__);
    e->type = UI_LOCAL_CLICK;
    e->location = rec;
    e->callback = callback;
    
    return e;
}

UIevent *UIcreateGlobalClickEvent(void (*callback)(void)) {
    UIevent *e = MEM_calloc(sizeof(UIevent), __func__);
    e->type = UI_GLOBAL_CLICK;
    e->callback = callback;
    
    return e;
}

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static _Bool check_in_list(UIevent *e) {
    UIevent *tmp = events_head;
    
    do {
        if (e==tmp) return true;
        tmp = tmp->next;
    } while (tmp != events_head);
    
    return false;
}
