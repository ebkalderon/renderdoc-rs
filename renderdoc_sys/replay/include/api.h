#ifndef EXTERN_H
#define EXTERN_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/replay_enums.h"

class Camera;
class CaptureFile;
struct CaptureOptions;
struct EnvironmentModification;
class RemoteServer;
class TargetControl;

typedef uint32_t bool32;

namespace RENDERDOC {
    Camera *InitCamera(CameraType type);

    CaptureFile *OpenCaptureFile(const char *logfile);

    TargetControl *CreateTargetControl(const char *host, uint32_t ident,
                                       const char *clientName,
                                       bool32 forceConnection);

    ReplayStatus CreateRemoteServerConnection(const char *host, uint32_t port,
                                              RemoteServer **rend);
}

//////////////////////////////////////////////////////////////////////////
// Maths/format/misc related exports
//////////////////////////////////////////////////////////////////////////

float RENDERDOC_HalfToFloat(uint16_t half);

uint16_t RENDERDOC_FloatToHalf(float flt);

uint32_t RENDERDOC_NumVerticesPerPrimitive(Topology topology);

uint32_t RENDERDOC_VertexOffset(Topology topology, uint32_t primitive);

//////////////////////////////////////////////////////////////////////////
// Target Control
//////////////////////////////////////////////////////////////////////////

uint32_t RENDERDOC_EnumerateRemoteTargets(const char *host, uint32_t nextIdent);

//////////////////////////////////////////////////////////////////////////
// Remote server
//////////////////////////////////////////////////////////////////////////

uint32_t RENDERDOC_GetDefaultRemoteServerPort();

void RENDERDOC_BecomeRemoteServer(const char *listenhost, uint32_t port,
                                  volatile bool32 *killReplay);

//////////////////////////////////////////////////////////////////////////
// Injection/execution capture functions.
//////////////////////////////////////////////////////////////////////////

void RENDERDOC_GetDefaultCaptureOptions(CaptureOptions *opts);

bool32 RENDERDOC_StartGlobalHook(const char *pathmatch, const char *logfile,
                                 const CaptureOptions &opts);

void RENDERDOC_StopGlobalHook();

bool32 RENDERDOC_IsGlobalHookActive();

bool32 RENDERDOC_CanGlobalHook();

uint32_t RENDERDOC_ExecuteAndInject(
    const char *app,
    const char *workingDir,
    const char *cmdLine,
    const rdctype::array<EnvironmentModification> &env,
    const char *logfile,
    const CaptureOptions &opts,
    bool32 waitForExit
);

uint32_t  RENDERDOC_InjectIntoProcess(
    uint32_t pid,
    const rdctype::array<EnvironmentModification> &env,
    const char *logfile,
    const CaptureOptions &opts,
    bool32 waitForExit
);

void RENDERDOC_StartSelfHostCapture(const char *dllname);

void RENDERDOC_EndSelfHostCapture(const char *dllname);

//////////////////////////////////////////////////////////////////////////
// Vulkan layer handling
//////////////////////////////////////////////////////////////////////////

bool RENDERDOC_NeedVulkanLayerRegistration(
    VulkanLayerFlags *flags,
    rdctype::array<rdctype::str> *myJSONs,
    rdctype::array<rdctype::str> *otherJSONs
);

void RENDERDOC_UpdateVulkanLayerRegistration(bool systemLevel);

//////////////////////////////////////////////////////////////////////////
// Miscellaneous!
//////////////////////////////////////////////////////////////////////////

void RENDERDOC_InitGlobalEnv(GlobalEnvironment env,
                             const rdctype::array<rdctype::str> &args);

void RENDERDOC_TriggerExceptionHandler(void *exceptionPtrs, bool32 crashed);

void RENDERDOC_SetDebugLogFile(const char *filename);

const char *RENDERDOC_GetLogFile();

void RENDERDOC_LogText(const char *text);

void RENDERDOC_LogMessage(LogType type, const char *project, const char *file,
                          unsigned int line, const char *text);

const char *RENDERDOC_GetVersionString();

const char *RENDERDOC_GetConfigSetting(const char *name);

void RENDERDOC_SetConfigSetting(const char *name, const char *value);

void RENDERDOC_GetAndroidFriendlyName(const rdctype::str &device,
                                      rdctype::str &friendly);

void RENDERDOC_EnumerateAndroidDevices(rdctype::str *deviceList);

void RENDERDOC_StartAndroidRemoteServer(const char *device);

void RENDERDOC_CheckAndroidPackage(const char *host, const char *exe,
                                   AndroidFlags *flags);

bool RENDERDOC_AddLayerToAndroidPackage(const char *host, const char *exe,
                                        float *progress);

#endif // EXTERN_H
