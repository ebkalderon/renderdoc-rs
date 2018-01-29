#include "../../renderdoc/renderdoc/api/replay/renderdoc_replay.h"

#include "../include/ReplayController.h"

APIProperties ReplayController::GetAPIProperties() {

}

rdctype::array<WindowingSystem> ReplayController::GetSupportedWindowSystems() {

}

ReplayOutput *ReplayController::CreateOutput(WindowingSystem system, void *data,
                                             ReplayOutputType type)
{
    IReplayOutput *out = this->inner->CreateOutput(system, data, type);

    return NULL;
}
