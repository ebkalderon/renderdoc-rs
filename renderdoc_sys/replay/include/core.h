#ifndef CORE_H
#define CORE_H

#include <cstddef>
#include <cstdint>

template <typename T>
inline const char *TypeName();

#define DECLARE_REFLECTION_STRUCT(type) \
  template <>                           \
  inline const char *TypeName<type>()   \
  {                                     \
    return #type;                       \
  }

typedef uint8_t byte;
typedef uint32_t bool32;

#ifndef DOCUMENT
#define DOCUMENT(text)
#endif

#if defined(RENDERDOC_PLATFORM_WIN32)

    #ifdef RENDERDOC_EXPORTS
        #define RENDERDOC_API __declspec(dllexport)
    #else
        #define RENDERDOC_API __declspec(dllimport)
    #endif
    #define RENDERDOC_CC __cdecl

#elif defined(RENDERDOC_PLATFORM_LINUX) || defined(RENDERDOC_PLATFORM_APPLE) || \
    defined(RENDERDOC_PLATFORM_ANDROID)

    #ifdef RENDERDOC_EXPORTS
        #define RENDERDOC_API __attribute__((visibility("default")))
    #else
        #define RENDERDOC_API
    #endif
    #define RENDERDOC_CC

#else
    #error "Unknown platform"
#endif

// windowing structures

#if defined(RENDERDOC_PLATFORM_WIN32)
// Win32 uses HWND
#endif

#if defined(RENDERDOC_WINDOWING_XLIB)
    // can't include xlib.h here as it defines a ton of crap like None
    // and Bool etc which can interfere with other headers
    typedef struct _XDisplay Display;
    typedef unsigned long Drawable;

    struct XlibWindowData {
        Display *display;
        Drawable window;
    };
#else
    typedef struct _XDisplay Display;
#endif

#if defined(RENDERDOC_WINDOWING_XCB)
    struct xcb_connection_t;
    typedef uint32_t xcb_window_t;

    struct XCBWindowData {
        xcb_connection_t *connection;
        xcb_window_t window;
    };
#endif

#if defined(RENDERDOC_PLATFORM_ANDROID)
    // android uses ANativeWindow*
#endif

enum class WindowingSystem : uint32_t {
    Unknown,
    Win32,
    Xlib,
    XCB,
    Android,
};

struct GlobalEnvironment {
    Display *xlibDisplay = NULL;
};

// needs to be declared up here for reference in basic_types

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_FreeArrayMem(const void *mem);
typedef void(RENDERDOC_CC *pRENDERDOC_FreeArrayMem)(const void *mem);

extern "C" RENDERDOC_API void *RENDERDOC_CC RENDERDOC_AllocArrayMem(uint64_t sz);
typedef void *(RENDERDOC_CC *pRENDERDOC_AllocArrayMem)(uint64_t sz);

#ifdef RENDERDOC_EXPORTS
    struct ResourceId;

    namespace ResourceIDGen {
        // the only function allowed access to ResourceId internals, for
        // allocating a new ID
        ResourceId GetNewUniqueID();
    };
#endif

// We give every resource a globally unique ID so that we can differentiate
// between two textures allocated in the same memory (after the first is freed)
//
// it's a struct around a uint64_t to aid in template selection
struct ResourceId {
    ResourceId() : id() {}
    inline static ResourceId Null() { return ResourceId(); }
    bool operator==(const ResourceId u) const { return id == u.id; }
    bool operator!=(const ResourceId u) const { return id != u.id; }
    bool operator<(const ResourceId u) const { return id < u.id; }
private:
    uint64_t id;

#ifdef RENDERDOC_EXPORTS
    friend ResourceId ResourceIDGen::GetNewUniqueID();
#endif
};

DECLARE_REFLECTION_STRUCT(ResourceId);

#endif // CORE_H
