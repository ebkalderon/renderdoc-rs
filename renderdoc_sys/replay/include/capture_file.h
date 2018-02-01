#ifndef CAPTURE_FILE_H
#define CAPTURE_FILE_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/replay_enums.h"

class ReplayController;
class ICaptureFile;

class CaptureFile {
public:
    CaptureFile(ICaptureFile *inner);
    void Shutdown();

    ReplayStatus OpenStatus();
    const char *Filename();

    ReplaySupport LocalReplaySupport();
    const char *DriverName();
    const char *RecordedMachineIdent();

    rdctype::pair<ReplayStatus, ReplayController> OpenCapture(float *progress);
    rdctype::array<byte> GetThumbnail(FileType type, uint32_t maxsize);

private:
    ICaptureFile *inner;
};

#endif // CAPTURE_FILE_H
