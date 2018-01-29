#ifndef REMOTE_SERVER_H
#define REMOTE_SERVER_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/replay_enums.h"

struct CaptureOptions;
struct EnvironmentModification;
struct PathEntry;
class ReplayController;

class RemoteServer {
public:
    static void BecomeRemoteServer(const char *listenhost, uint32_t port,
                                   uint32_t *kill);

    static uint32_t ExecuteAndInject(
        const char *app,
        const char *workingDir,
        const char *cmdLine,
        const rdctype::array<EnvironmentModification> &env,
        const char *logfile,
        const CaptureOptions &opts,
        bool32 waitForExit
    );

    static uint32_t GetDefaultRemoteServerPort();

    RemoteServer(const char *host, uint32_t port);
    ~RemoteServer();

    void ShutdownConnection();
    bool Ping();

    rdctype::array<rdctype::str> LocalProxies();
    rdctype::array<rdctype::str> RemoteSupportedReplays();

    rdctype::str GetHomeFolder();
    rdctype::array<PathEntry> ListFolder(const char *path);

    void TakeOwnershipCapture(const char *filename);
    rdctype::str CopyCaptureToRemote(const char *filename, float *progress);
    void CopyCaptureFromRemote(const char *remotepath, const char *localpath,
                               float *progress);

    rdctype::pair<ReplayStatus, ReplayController*> OpenCapture(
        uint32_t proxyid,
        const char *logfile,
        float *progress
    );

    void CloseCapture(ReplayController *ctrl);

    static const uint32_t NoPreference = ~0U;

private:
    void ShutdownServerAndConnection();

    class IRemoteServer *inner;
};

#endif // REMOTE_SERVER_H
