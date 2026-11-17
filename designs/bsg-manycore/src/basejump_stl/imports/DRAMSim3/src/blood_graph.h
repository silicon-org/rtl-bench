// Author: Tommy Jung

#ifndef __BLOOD_GRAPH_H
#define __BLOOD_GRAPH_H

#include <fstream>
#include <vector>
#include "common.h"
#include "configuration.h"
#include "command_queue.h"
#include "channel_state.h"

namespace dramsim3 {

class BloodGraph {

  public:
    BloodGraph(int channel_id, const Config &config);
    void ClockTick();
    void IsInRefresh(bool ref);
    void IssueCommand(const Command &cmd);
    CommandQueue* cmd_queue_;
    ChannelState* channel_state_;

    void PrintTagStats(uint32_t tag);

  private:
    // channel info
    const Config &config_;
    int channel_id_;
    int clk_;
    int bank_count_;

    // refresh info
    bool is_in_ref_;
    int ref_count_;

    // bank info
    std::vector<bool> read_issued_;
    std::vector<bool> write_issued_;
    std::vector<int> pre_count_;
    std::vector<int> act_count_; 

    // trace stream
    std::ofstream trace_;
    void PrintTrace(int bank_id, const std::string &str);

    // tracking info
    int data_line_busy_read_;  
    int data_line_busy_write_;  
    bool cmd_issued_;

    // stat info
    int stat_busy_count_;
    int stat_read_count_;
    int stat_write_count_;
    int stat_refresh_count_;
    std::ofstream stat_;

    // periodic stat;
    const int stat_period_;
    bool kernel_start_received_;
    std::ofstream periodic_stat_;
    void PrintPeriodicStats();
};

} // namespace dramsim3

#endif
