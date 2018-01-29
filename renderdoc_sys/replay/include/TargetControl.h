#ifndef TARGET_CONTROL_H
#define TARGET_CONTROL_H

#include <cstdint>

class TargetControlMessage;

class TargetControl {
public:
    TargetControl(const char *host, uint32_t ident, const char *clientName,
                  bool forceConnection);
    ~TargetControl();

    bool Connected();

    const char *GetTarget();
    const char *GetAPI();
    uint32_t GetPID();
    const char *GetBusyClient();

    void TriggerCapture(uint32_t numFrames);
    void QueueCapture(uint32_t frameNumber);
    void CopyCapture(uint32_t remoteID, const char *localpath);
    void DeleteCapture(uint32_t remoteID);

    TargetControlMessage ReceiveMessage();

private:
    void Shutdown();

    class ITargetControl *inner;
};

#endif // TARGET_CONTROL_H
