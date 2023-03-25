#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
typedef struct _Dart_Handle* Dart_Handle;

typedef struct DartCObject DartCObject;

typedef int64_t DartPort;

typedef bool (*DartPostCObjectFnType)(DartPort port_id, void *message);

typedef struct wire_uint_8_list {
  uint8_t *ptr;
  int32_t len;
} wire_uint_8_list;

typedef struct wire_Event_AddTodo {
  struct wire_uint_8_list *field0;
} wire_Event_AddTodo;

typedef struct wire_Event_RemoveTodo {
  uintptr_t field0;
} wire_Event_RemoveTodo;

typedef struct wire_Event_CleanList {

} wire_Event_CleanList;

typedef union EventKind {
  struct wire_Event_AddTodo *AddTodo;
  struct wire_Event_RemoveTodo *RemoveTodo;
  struct wire_Event_CleanList *CleanList;
} EventKind;

typedef struct wire_Event {
  int32_t tag;
  union EventKind *kind;
} wire_Event;

typedef struct DartCObject *WireSyncReturn;

void store_dart_post_cobject(DartPostCObjectFnType ptr);

Dart_Handle get_dart_object(uintptr_t ptr);

void drop_dart_object(uintptr_t ptr);

uintptr_t new_dart_opaque(Dart_Handle handle);

intptr_t init_frb_dart_api_dl(void *obj);

void wire_process_event(int64_t port_, struct wire_Event *event);

void wire_view(int64_t port_);

struct wire_Event *new_box_autoadd_event_0(void);

struct wire_uint_8_list *new_uint_8_list_0(int32_t len);

union EventKind *inflate_Event_AddTodo(void);

union EventKind *inflate_Event_RemoveTodo(void);

void free_WireSyncReturn(WireSyncReturn ptr);

static int64_t dummy_method_to_enforce_bundling(void) {
    int64_t dummy_var = 0;
    dummy_var ^= ((int64_t) (void*) wire_process_event);
    dummy_var ^= ((int64_t) (void*) wire_view);
    dummy_var ^= ((int64_t) (void*) new_box_autoadd_event_0);
    dummy_var ^= ((int64_t) (void*) new_uint_8_list_0);
    dummy_var ^= ((int64_t) (void*) inflate_Event_AddTodo);
    dummy_var ^= ((int64_t) (void*) inflate_Event_RemoveTodo);
    dummy_var ^= ((int64_t) (void*) free_WireSyncReturn);
    dummy_var ^= ((int64_t) (void*) store_dart_post_cobject);
    dummy_var ^= ((int64_t) (void*) get_dart_object);
    dummy_var ^= ((int64_t) (void*) drop_dart_object);
    dummy_var ^= ((int64_t) (void*) new_dart_opaque);
    return dummy_var;
}
