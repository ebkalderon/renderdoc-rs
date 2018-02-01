#ifndef REMOTE_SERVER_H
#define REMOTE_SERVER_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/replay_enums.h"

struct CaptureOptions;
struct EnvironmentModification;
class IRemoteServer;
struct PathEntry;
class ReplayController;

class RemoteServer {
public:
    RemoteServer(IRemoteServer *inner);
    void ShutdownServerAndConnection();

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

    rdctype::pair<ReplayStatus, ReplayController> OpenCapture(
        uint32_t proxyid,
        const char *logfile,
        float *progress
    );

    void CloseCapture(ReplayController *ctrl);

    static const uint32_t NoPreference = ~0U;

private:
    IRemoteServer *inner;
};

#endif // REMOTE_SERVER_H
