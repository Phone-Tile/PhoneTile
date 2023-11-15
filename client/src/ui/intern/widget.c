//
//  widget.c
//
//  Created by Emmanuel Mera on 08/11/2023.
//

#include <assert.h>

#include "widget.h"
#include "MEM_alloc.h"

#define MAX(a, b) (a < b) ? (b) : (a)

/* ---------------------------- */
/*  local variables             */
/* ---------------------------- */

static UIwidget *widgets_head = NULL;

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static _Bool check_in_list(UIwidget *w);

/* ---------------------------- */
/*  implementation              */
/* ---------------------------- */

UIwidget *UIcreateButtonWidget(Rectangle rec, void (*callback)(void)) {
    UIwidget *w = MEM_calloc(sizeof(UIwidget), __func__);
    
    w->type = UI_BUTTON;
    w->location = rec;
    w->event = UIcreateLocalClickEvent(rec, callback);
    
    return w;
}

void UIrenderButton(UIwidget *w) {
    Rectangle rec = w->location;
    
    float offset = MAX(rec.width, rec.height);
    
    rec.x += offset/20.;
    rec.y += offset/20.;
    DrawRectangleRounded(rec, .2f, 20, (Color){155, 155, 155, 255});
    
    rec.x = w->location.x;
    rec.y = w->location.y;
    DrawRectangleRounded(rec, .2f, 20, WHITE);
    
    rec.x += offset/40;
    rec.y += offset/40;
    rec.width -= offset/20;
    rec.height -= offset/20;
    DrawRectangle(rec.x, rec.y, rec.width, rec.height, BLACK);
}

void UIrenderWidgets(void) {
    if (!widgets_head) return;
    
    UIwidget *w = widgets_head;
    
    do {
        switch (w->type) {
            case UI_BUTTON:
                UIrenderButton(w);
                break;
                
            default:
                break;
        }
        w = w->next;
    } while (w != widgets_head);
}

void UIaddWidget(UIwidget *w) {
    if (widgets_head) {
        w->next = widgets_head;
        w->prev = widgets_head->prev;
        
        widgets_head->prev->next = w;
        widgets_head->prev = w;
        widgets_head = w;
    } else {
        widgets_head = w;
        w->next = w;
        widgets_head->prev = w;
    }
    UIaddEvent(w->event);
}

void UIremoveWidget(UIwidget *w) {
    assert(check_in_list(w));
    
    if (w->next == w) {
        widgets_head = NULL;
    }
    
    if (widgets_head == w) {
        widgets_head = w->next;
        w->next->prev = w->prev;
        w->prev->next = w->next;
    }
}

/* ---------------------------- */
/*  local functions             */
/* ---------------------------- */

static _Bool check_in_list(UIwidget *w) {
    UIwidget *tmp = widgets_head;
    
    do {
        if (w==tmp) return true;
        tmp = tmp->next;
    } while (tmp != widgets_head);
    
    return false;
}
