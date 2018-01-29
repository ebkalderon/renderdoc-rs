#ifndef EXTERN_H
#define EXTERN_H

#include <cstdint>

class ICamera;
class ICaptureFile;
class IRemoteServer;
class ITargetControl;
enum class LogType;

extern "C" RENDERDOC_API ICamera *RENDERDOC_CC RENDERDOC_InitCamera(CameraType type);

//////////////////////////////////////////////////////////////////////////
// Maths/format/misc related exports
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API float RENDERDOC_CC RENDERDOC_HalfToFloat(uint16_t half);

extern "C" RENDERDOC_API uint16_t RENDERDOC_CC RENDERDOC_FloatToHalf(float flt);

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC RENDERDOC_NumVerticesPerPrimitive(Topology topology);

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC RENDERDOC_VertexOffset(Topology topology,
                                                                      uint32_t primitive);

//////////////////////////////////////////////////////////////////////////
// Create a capture handle.
//
// Takes the filename of the log. Always returns a valid handle that must be shut down.
// If any errors happen this can be queried with :meth:`CaptureFile.Status`.
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API ICaptureFile *RENDERDOC_CC RENDERDOC_OpenCaptureFile(const char *logfile);

//////////////////////////////////////////////////////////////////////////
// Target Control
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API ITargetControl *RENDERDOC_CC RENDERDOC_CreateTargetControl(
    const char *host, uint32_t ident, const char *clientName, bool32 forceConnection);

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC RENDERDOC_EnumerateRemoteTargets(const char *host,
                                                                                uint32_t nextIdent);

//////////////////////////////////////////////////////////////////////////
// Remote server
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC RENDERDOC_GetDefaultRemoteServerPort();

extern "C" RENDERDOC_API ReplayStatus RENDERDOC_CC
RENDERDOC_CreateRemoteServerConnection(const char *host, uint32_t port, IRemoteServer **rend);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_BecomeRemoteServer(const char *listenhost,
                                                                        uint32_t port,
                                                                        volatile bool32 *killReplay);

//////////////////////////////////////////////////////////////////////////
// Injection/execution capture functions.
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_GetDefaultCaptureOptions(CaptureOptions *opts);

extern "C" RENDERDOC_API bool32 RENDERDOC_CC RENDERDOC_StartGlobalHook(const char *pathmatch,
                                                                       const char *logfile,
                                                                       const CaptureOptions &opts);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_StopGlobalHook();

extern "C" RENDERDOC_API bool32 RENDERDOC_CC RENDERDOC_IsGlobalHookActive();

extern "C" RENDERDOC_API bool32 RENDERDOC_CC RENDERDOC_CanGlobalHook();

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC
RENDERDOC_ExecuteAndInject(const char *app, const char *workingDir, const char *cmdLine,
                           const rdctype::array<EnvironmentModification> &env, const char *logfile,
                           const CaptureOptions &opts, bool32 waitForExit);

extern "C" RENDERDOC_API uint32_t RENDERDOC_CC
RENDERDOC_InjectIntoProcess(uint32_t pid, const rdctype::array<EnvironmentModification> &env,
                            const char *logfile, const CaptureOptions &opts, bool32 waitForExit);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_StartSelfHostCapture(const char *dllname);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_EndSelfHostCapture(const char *dllname);

//////////////////////////////////////////////////////////////////////////
// Vulkan layer handling
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API bool RENDERDOC_CC
RENDERDOC_NeedVulkanLayerRegistration(VulkanLayerFlags *flags, rdctype::array<rdctype::str> *myJSONs,
                                      rdctype::array<rdctype::str> *otherJSONs);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_UpdateVulkanLayerRegistration(bool systemLevel);

//////////////////////////////////////////////////////////////////////////
// Miscellaneous!
//////////////////////////////////////////////////////////////////////////

extern "C" RENDERDOC_API void RENDERDOC_CC
RENDERDOC_InitGlobalEnv(GlobalEnvironment env, const rdctype::array<rdctype::str> &args);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_TriggerExceptionHandler(void *exceptionPtrs,
                                                                             bool32 crashed);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_SetDebugLogFile(const char *filename);

extern "C" RENDERDOC_API const char *RENDERDOC_CC RENDERDOC_GetLogFile();

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_LogText(const char *text);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_LogMessage(LogType type, const char *project,
                                                                const char *file, unsigned int line,
                                                                const char *text);

extern "C" RENDERDOC_API const char *RENDERDOC_CC RENDERDOC_GetVersionString();

extern "C" RENDERDOC_API const char *RENDERDOC_CC RENDERDOC_GetConfigSetting(const char *name);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_SetConfigSetting(const char *name,
                                                                      const char *value);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_GetAndroidFriendlyName(const rdctype::str &device,
                                                                            rdctype::str &friendly);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_EnumerateAndroidDevices(rdctype::str *deviceList);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_StartAndroidRemoteServer(const char *device);

extern "C" RENDERDOC_API void RENDERDOC_CC RENDERDOC_CheckAndroidPackage(const char *host,
                                                                         const char *exe,
                                                                         AndroidFlags *flags);

extern "C" RENDERDOC_API bool RENDERDOC_CC RENDERDOC_AddLayerToAndroidPackage(const char *host,
                                                                              const char *exe,
                                                                              float *progress);

#endif // EXTERN_H
