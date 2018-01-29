#ifndef CAPTURE_FILE_H
#define CAPTURE_FILE_H

#include <cstdint>

#include "../../renderdoc/renderdoc/api/replay/basic_types.h"

typedef uint8_t byte;

enum class FileType : uint32_t;
class ReplayController;
enum class ReplayStatus : uint32_t;
enum class ReplaySupport : uint32_t;

class CaptureFile {
public:
    CaptureFile(const char *logfile);
    ~CaptureFile();

    ReplayStatus OpenStatus();
    const char *Filename();

    ReplaySupport LocalReplaySupport();
    const char *DriverName();
    const char *RecordedMachineIdent();

    rdctype::pair<ReplayStatus, ReplayController> OpenCapture(float *progress);
    rdctype::array<byte> GetThumbnail(FileType type, uint32_t maxsize);

private:
    void Shutdown();

    class ICaptureFile *inner;
};

#endif // CAPTURE_FILE_H
