#ifndef TARGET_CONTROL_H
#define TARGET_CONTROL_H

#include <cstdint>

class ITargetControl;
class TargetControlMessage;

class TargetControl {
public:
    TargetControl(ITargetControl *inner);
    void Shutdown();

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
    ITargetControl *inner;
};

#endif // TARGET_CONTROL_H
