
#include"chisel_db.h"
#include <string.h>
#include <stdbool.h>

bool dump;
sqlite3 *mem_db;
char *zErrMsg;
int rc;

bool enable_dump_LoadPfMshr1_hart0 = true;

bool enable_dump_FTQTable0 = true;

bool enable_dump_TLLog = true;

bool enable_dump_LoadTraceMiss1_hart0 = true;

bool enable_dump_PageCache_hart0 = true;

bool enable_dump_FetchToIBuffer0 = true;

bool enable_dump_LoadTraceMiss0_hart0 = true;

bool enable_dump_L2TlbPrefetch_hart0 = true;

bool enable_dump_LoadTraceMiss2_hart0 = true;

bool enable_dump_LoadPfTrace0_hart0 = true;

bool enable_dump_PTW_hart0 = true;

bool enable_dump_Tip_0 = true;

bool enable_dump_LoadTrace1_hart0 = true;

bool enable_dump_LoadPfMshr0_hart0 = true;

bool enable_dump_L2TlbMissQueue_hart0 = true;

bool enable_dump_L1Tlb_hart0 = true;

bool enable_dump_LoadPfTrace1_hart0 = true;

bool enable_dump_BankConflict0 = true;

bool enable_dump_LoadAccessDB0 = true;

bool enable_dump_LoadTrace0_hart0 = true;

bool enable_dump_ICacheSRAMTable0 = true;

bool enable_dump_LoadPfTrace2_hart0 = true;

bool enable_dump_L1MissQMissTrace_hart0 = true;

bool enable_dump_L2MP = true;

bool enable_dump_IfuWbToFtq0 = true;

bool enable_dump_LoadPfMshr2_hart0 = true;

bool enable_dump_LoadMissDB0 = true;

bool enable_dump_InstTable0 = true;

bool enable_dump_LoadTrace2_hart0 = true;



static int callback(void *NotUsed, int argc, char **argv, char **azColName){
  return 0;
}

  
void init_db_L2TlbPrefetch_hart0() {}

  
void init_db_L2TlbMissQueue_hart0() {}

  
void init_db_LoadMissDB0() {}

  
void init_db_InstTable0() {}

  
void init_db_LoadPfMshr1_hart0() {}

  
void init_db_FTQTable0() {}

  
void init_db_PageCache_hart0() {}

  
void init_db_LoadTrace1_hart0() {}

  
void init_db_LoadTraceMiss0_hart0() {}

  
void init_db_LoadAccessDB0() {}

  
void init_db_LoadPfTrace1_hart0() {}

  
void init_db_IfuWbToFtq0() {}

  
void init_db_L1Tlb_hart0() {}

  
void init_db_FetchToIBuffer0() {}

  
void init_db_LoadTraceMiss2_hart0() {}

  
void init_db_BankConflict0() {}

  
void init_db_LoadTrace0_hart0() {}

  
void init_db_LoadPfMshr2_hart0() {}

  
void init_db_LoadTraceMiss1_hart0() {}

  
void init_db_ICacheSRAMTable0() {}

  
void init_db_L2MP() {}

  
void init_db_LoadPfTrace2_hart0() {}

  
void init_db_LoadPfMshr0_hart0() {}

  
void init_db_L1MissQMissTrace_hart0() {}

  
void init_db_PTW_hart0() {}

  
void init_db_LoadTrace2_hart0() {}

  
void init_db_TLLog() {}

  
void init_db_LoadPfTrace0_hart0() {}

  
void init_db_Tip_0() {}


  
extern "C" void L2TlbPrefetch_hart0_write(
  uint64_t vpn,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void L2TlbMissQueue_hart0_write(
  uint64_t vpn,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadMissDB0_write(
  uint64_t missState,
  uint64_t vaddr,
  uint64_t paddr,
  uint64_t robIdx,
  uint64_t timeCnt,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void InstTable0_write(
  uint64_t exceptType,
  uint64_t lsInfo_replayCnt,
  uint64_t lsInfo_replayCause_0,
  uint64_t lsInfo_replayCause_1,
  uint64_t lsInfo_replayCause_2,
  uint64_t lsInfo_replayCause_3,
  uint64_t lsInfo_replayCause_4,
  uint64_t lsInfo_replayCause_5,
  uint64_t lsInfo_replayCause_6,
  uint64_t lsInfo_replayCause_7,
  uint64_t lsInfo_replayCause_8,
  uint64_t lsInfo_replayCause_9,
  uint64_t lsInfo_s3_isReplay,
  uint64_t lsInfo_s3_isReplayRS,
  uint64_t lsInfo_s3_isReplaySlow,
  uint64_t lsInfo_s3_isReplayFast,
  uint64_t lsInfo_s2_isForwardFail,
  uint64_t lsInfo_s2_isDcacheFirstMiss,
  uint64_t lsInfo_s2_isBankConflict,
  uint64_t lsInfo_s1_isLoadToLoadForward,
  uint64_t lsInfo_s1_isTlbFirstMiss,
  uint64_t tlbLatency,
  uint64_t rsFuLatency,
  uint64_t executeLatency,
  uint64_t issueLatency,
  uint64_t selectLatency,
  uint64_t enqRsLatency,
  uint64_t dispatchLatency,
  uint64_t writebackTime,
  uint64_t issueTime,
  uint64_t dpaddr,
  uint64_t dvaddr,
  uint64_t robIdx,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfMshr1_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void FTQTable0_write(
  uint64_t predStage,
  uint64_t isTaken,
  uint64_t misPred,
  uint64_t isRet,
  uint64_t isCall,
  uint64_t isJmp,
  uint64_t isBr,
  uint64_t target,
  uint64_t pc,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void PageCache_hart0_write(
  uint64_t hit,
  uint64_t l1Hit,
  uint64_t l2Hit,
  uint64_t prefetch,
  uint64_t prefetched,
  uint64_t is_first,
  uint64_t bypassed,
  uint64_t source,
  uint64_t vpn,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTrace1_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTraceMiss0_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadAccessDB0_write(
  uint64_t real_way_num,
  uint64_t dm_way_num,
  uint64_t pred_way_num,
  uint64_t missState,
  uint64_t vaddr,
  uint64_t paddr,
  uint64_t robIdx,
  uint64_t timeCnt,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfTrace1_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void IfuWbToFtq0_write(
  uint64_t checkInvalidTaken,
  uint64_t checkNotCFIFault,
  uint64_t checkTargetFault,
  uint64_t checkRetFault,
  uint64_t checkJalFault,
  uint64_t miss_pred_offset,
  uint64_t is_miss_pred,
  uint64_t start_addr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void L1Tlb_hart0_write(
  uint64_t vpn,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void FetchToIBuffer0_write(
  uint64_t is_cache_hit,
  uint64_t exception,
  uint64_t instr_count,
  uint64_t start_addr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTraceMiss2_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void BankConflict0_write(
  uint64_t fake_rr_bank_conflict,
  uint64_t way_index,
  uint64_t bank_index_0,
  uint64_t bank_index_1,
  uint64_t set_index_0,
  uint64_t set_index_1,
  uint64_t set_index_2,
  uint64_t addr_0,
  uint64_t addr_1,
  uint64_t addr_2,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTrace0_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfMshr2_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTraceMiss1_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void ICacheSRAMTable0_write(
  uint64_t waymask,
  uint64_t vSetIdx,
  uint64_t blkPaddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void L2MP_write(
  uint64_t metaWway,
  uint64_t metaWvalid,
  uint64_t mshrId,
  uint64_t allocPtr,
  uint64_t allocValid,
  uint64_t dirWay,
  uint64_t dirHit,
  uint64_t sset,
  uint64_t tag,
  uint64_t opcode,
  uint64_t channel,
  uint64_t mshrTask,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfTrace2_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfMshr0_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void L1MissQMissTrace_hart0_write(
  uint64_t pc,
  uint64_t source,
  uint64_t paddr,
  uint64_t vaddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void PTW_hart0_write(
  uint64_t source,
  uint64_t vpn,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadTrace2_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void TLLog_write(
  uint64_t echo,
  uint64_t user,
  uint64_t data_0,
  uint64_t data_1,
  uint64_t data_2,
  uint64_t data_3,
  uint64_t address,
  uint64_t sink,
  uint64_t source,
  uint64_t param,
  uint64_t opcode,
  uint64_t channel,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void LoadPfTrace0_hart0_write(
  uint64_t paddr,
  uint64_t stamp,
  char *site
) {}

  
extern "C" void Tip_0_write(
  uint64_t debugLsInfo_replayCnt,
  uint64_t debugLsInfo_replayCause_0,
  uint64_t debugLsInfo_replayCause_1,
  uint64_t debugLsInfo_replayCause_2,
  uint64_t debugLsInfo_replayCause_3,
  uint64_t debugLsInfo_replayCause_4,
  uint64_t debugLsInfo_replayCause_5,
  uint64_t debugLsInfo_replayCause_6,
  uint64_t debugLsInfo_replayCause_7,
  uint64_t debugLsInfo_replayCause_8,
  uint64_t debugLsInfo_replayCause_9,
  uint64_t debugLsInfo_s3_isReplay,
  uint64_t debugLsInfo_s3_isReplayRS,
  uint64_t debugLsInfo_s3_isReplaySlow,
  uint64_t debugLsInfo_s3_isReplayFast,
  uint64_t debugLsInfo_s2_isForwardFail,
  uint64_t debugLsInfo_s2_isDcacheFirstMiss,
  uint64_t debugLsInfo_s2_isBankConflict,
  uint64_t debugLsInfo_s1_isLoadToLoadForward,
  uint64_t debugLsInfo_s1_isTlbFirstMiss,
  uint64_t redirect_pc,
  uint64_t redirect_bits_debugIsMemVio,
  uint64_t redirect_bits_debugIsCtrl,
  uint64_t redirect_bits_debug_runahead_checkpoint_id,
  uint64_t redirect_bits_stFtqOffset,
  uint64_t redirect_bits_stFtqIdx_value,
  uint64_t redirect_bits_stFtqIdx_flag,
  uint64_t redirect_bits_fullTarget,
  uint64_t redirect_bits_cfiUpdate_backendIAF,
  uint64_t redirect_bits_cfiUpdate_backendIPF,
  uint64_t redirect_bits_cfiUpdate_backendIGPF,
  uint64_t redirect_bits_cfiUpdate_addIntoHist,
  uint64_t redirect_bits_cfiUpdate_shift,
  uint64_t redirect_bits_cfiUpdate_isMisPred,
  uint64_t redirect_bits_cfiUpdate_taken,
  uint64_t redirect_bits_cfiUpdate_target,
  uint64_t redirect_bits_cfiUpdate_predTaken,
  uint64_t redirect_bits_cfiUpdate_sc_hit,
  uint64_t redirect_bits_cfiUpdate_jr_hit,
  uint64_t redirect_bits_cfiUpdate_br_hit,
  uint64_t redirect_bits_cfiUpdate_specCnt_0,
  uint64_t redirect_bits_cfiUpdate_specCnt_1,
  uint64_t redirect_bits_cfiUpdate_histPtr_value,
  uint64_t redirect_bits_cfiUpdate_histPtr_flag,
  uint64_t redirect_bits_cfiUpdate_ghr,
  uint64_t redirect_bits_cfiUpdate_lastBrNumOH,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_0_bits_0,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_0_bits_1,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_0_bits_2,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_0_bits_3,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_1_bits_0,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_1_bits_1,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_1_bits_2,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_1_bits_3,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_2_bits_0,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_2_bits_1,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_2_bits_2,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_2_bits_3,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_3_bits_0,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_3_bits_1,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_3_bits_2,
  uint64_t redirect_bits_cfiUpdate_afhob_afhob_3_bits_3,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_0_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_1_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_2_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_3_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_4_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_5_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_6_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_7_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_8_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_9_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_10_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_11_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_12_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_13_folded_hist,
  uint64_t redirect_bits_cfiUpdate_folded_hist_hist_14_folded_hist,
  uint64_t redirect_bits_cfiUpdate_topAddr,
  uint64_t redirect_bits_cfiUpdate_NOS_value,
  uint64_t redirect_bits_cfiUpdate_NOS_flag,
  uint64_t redirect_bits_cfiUpdate_TOSR_value,
  uint64_t redirect_bits_cfiUpdate_TOSR_flag,
  uint64_t redirect_bits_cfiUpdate_TOSW_value,
  uint64_t redirect_bits_cfiUpdate_TOSW_flag,
  uint64_t redirect_bits_cfiUpdate_sctr,
  uint64_t redirect_bits_cfiUpdate_ssp,
  uint64_t redirect_bits_cfiUpdate_pd_isRet,
  uint64_t redirect_bits_cfiUpdate_pd_isCall,
  uint64_t redirect_bits_cfiUpdate_pd_brType,
  uint64_t redirect_bits_cfiUpdate_pd_isRVC,
  uint64_t redirect_bits_cfiUpdate_pd_valid,
  uint64_t redirect_bits_cfiUpdate_pc,
  uint64_t redirect_bits_interrupt,
  uint64_t redirect_bits_level,
  uint64_t redirect_bits_ftqOffset,
  uint64_t redirect_bits_ftqIdx_value,
  uint64_t redirect_bits_ftqIdx_flag,
  uint64_t redirect_bits_robIdx_value,
  uint64_t redirect_bits_robIdx_flag,
  uint64_t redirect_bits_isRVC,
  uint64_t redirect_valid,
  uint64_t commits_robIdx_0_value,
  uint64_t commits_robIdx_0_flag,
  uint64_t commits_robIdx_1_value,
  uint64_t commits_robIdx_1_flag,
  uint64_t commits_robIdx_2_value,
  uint64_t commits_robIdx_2_flag,
  uint64_t commits_robIdx_3_value,
  uint64_t commits_robIdx_3_flag,
  uint64_t commits_robIdx_4_value,
  uint64_t commits_robIdx_4_flag,
  uint64_t commits_robIdx_5_value,
  uint64_t commits_robIdx_5_flag,
  uint64_t commits_robIdx_6_value,
  uint64_t commits_robIdx_6_flag,
  uint64_t commits_robIdx_7_value,
  uint64_t commits_robIdx_7_flag,
  uint64_t commits_info_0_dirtyVs,
  uint64_t commits_info_0_dirtyFs,
  uint64_t commits_info_0_debug_fuType,
  uint64_t commits_info_0_debug_pdest,
  uint64_t commits_info_0_debug_ldest,
  uint64_t commits_info_0_debug_instr,
  uint64_t commits_info_0_debug_pc,
  uint64_t commits_info_0_traceBlockInPipe_ilastsize,
  uint64_t commits_info_0_traceBlockInPipe_iretire,
  uint64_t commits_info_0_traceBlockInPipe_itype,
  uint64_t commits_info_0_needFlush,
  uint64_t commits_info_0_isMove,
  uint64_t commits_info_0_loadWaitBit,
  uint64_t commits_info_0_rfWen,
  uint64_t commits_info_0_fpWen,
  uint64_t commits_info_0_instrSize,
  uint64_t commits_info_0_ftqOffset,
  uint64_t commits_info_0_ftqIdx_value,
  uint64_t commits_info_0_ftqIdx_flag,
  uint64_t commits_info_0_commitType,
  uint64_t commits_info_0_isVls,
  uint64_t commits_info_0_isHls,
  uint64_t commits_info_0_isVset,
  uint64_t commits_info_0_isRVC,
  uint64_t commits_info_0_vxsat,
  uint64_t commits_info_0_fflags,
  uint64_t commits_info_0_wflags,
  uint64_t commits_info_0_interrupt_safe,
  uint64_t commits_info_0_realDestSize,
  uint64_t commits_info_0_commit_w,
  uint64_t commits_info_0_commit_v,
  uint64_t commits_info_0_walk_v,
  uint64_t commits_info_1_dirtyVs,
  uint64_t commits_info_1_dirtyFs,
  uint64_t commits_info_1_debug_fuType,
  uint64_t commits_info_1_debug_pdest,
  uint64_t commits_info_1_debug_ldest,
  uint64_t commits_info_1_debug_instr,
  uint64_t commits_info_1_debug_pc,
  uint64_t commits_info_1_traceBlockInPipe_ilastsize,
  uint64_t commits_info_1_traceBlockInPipe_iretire,
  uint64_t commits_info_1_traceBlockInPipe_itype,
  uint64_t commits_info_1_needFlush,
  uint64_t commits_info_1_isMove,
  uint64_t commits_info_1_loadWaitBit,
  uint64_t commits_info_1_rfWen,
  uint64_t commits_info_1_fpWen,
  uint64_t commits_info_1_instrSize,
  uint64_t commits_info_1_ftqOffset,
  uint64_t commits_info_1_ftqIdx_value,
  uint64_t commits_info_1_ftqIdx_flag,
  uint64_t commits_info_1_commitType,
  uint64_t commits_info_1_isVls,
  uint64_t commits_info_1_isHls,
  uint64_t commits_info_1_isVset,
  uint64_t commits_info_1_isRVC,
  uint64_t commits_info_1_vxsat,
  uint64_t commits_info_1_fflags,
  uint64_t commits_info_1_wflags,
  uint64_t commits_info_1_interrupt_safe,
  uint64_t commits_info_1_realDestSize,
  uint64_t commits_info_1_commit_w,
  uint64_t commits_info_1_commit_v,
  uint64_t commits_info_1_walk_v,
  uint64_t commits_info_2_dirtyVs,
  uint64_t commits_info_2_dirtyFs,
  uint64_t commits_info_2_debug_fuType,
  uint64_t commits_info_2_debug_pdest,
  uint64_t commits_info_2_debug_ldest,
  uint64_t commits_info_2_debug_instr,
  uint64_t commits_info_2_debug_pc,
  uint64_t commits_info_2_traceBlockInPipe_ilastsize,
  uint64_t commits_info_2_traceBlockInPipe_iretire,
  uint64_t commits_info_2_traceBlockInPipe_itype,
  uint64_t commits_info_2_needFlush,
  uint64_t commits_info_2_isMove,
  uint64_t commits_info_2_loadWaitBit,
  uint64_t commits_info_2_rfWen,
  uint64_t commits_info_2_fpWen,
  uint64_t commits_info_2_instrSize,
  uint64_t commits_info_2_ftqOffset,
  uint64_t commits_info_2_ftqIdx_value,
  uint64_t commits_info_2_ftqIdx_flag,
  uint64_t commits_info_2_commitType,
  uint64_t commits_info_2_isVls,
  uint64_t commits_info_2_isHls,
  uint64_t commits_info_2_isVset,
  uint64_t commits_info_2_isRVC,
  uint64_t commits_info_2_vxsat,
  uint64_t commits_info_2_fflags,
  uint64_t commits_info_2_wflags,
  uint64_t commits_info_2_interrupt_safe,
  uint64_t commits_info_2_realDestSize,
  uint64_t commits_info_2_commit_w,
  uint64_t commits_info_2_commit_v,
  uint64_t commits_info_2_walk_v,
  uint64_t commits_info_3_dirtyVs,
  uint64_t commits_info_3_dirtyFs,
  uint64_t commits_info_3_debug_fuType,
  uint64_t commits_info_3_debug_pdest,
  uint64_t commits_info_3_debug_ldest,
  uint64_t commits_info_3_debug_instr,
  uint64_t commits_info_3_debug_pc,
  uint64_t commits_info_3_traceBlockInPipe_ilastsize,
  uint64_t commits_info_3_traceBlockInPipe_iretire,
  uint64_t commits_info_3_traceBlockInPipe_itype,
  uint64_t commits_info_3_needFlush,
  uint64_t commits_info_3_isMove,
  uint64_t commits_info_3_loadWaitBit,
  uint64_t commits_info_3_rfWen,
  uint64_t commits_info_3_fpWen,
  uint64_t commits_info_3_instrSize,
  uint64_t commits_info_3_ftqOffset,
  uint64_t commits_info_3_ftqIdx_value,
  uint64_t commits_info_3_ftqIdx_flag,
  uint64_t commits_info_3_commitType,
  uint64_t commits_info_3_isVls,
  uint64_t commits_info_3_isHls,
  uint64_t commits_info_3_isVset,
  uint64_t commits_info_3_isRVC,
  uint64_t commits_info_3_vxsat,
  uint64_t commits_info_3_fflags,
  uint64_t commits_info_3_wflags,
  uint64_t commits_info_3_interrupt_safe,
  uint64_t commits_info_3_realDestSize,
  uint64_t commits_info_3_commit_w,
  uint64_t commits_info_3_commit_v,
  uint64_t commits_info_3_walk_v,
  uint64_t commits_info_4_dirtyVs,
  uint64_t commits_info_4_dirtyFs,
  uint64_t commits_info_4_debug_fuType,
  uint64_t commits_info_4_debug_pdest,
  uint64_t commits_info_4_debug_ldest,
  uint64_t commits_info_4_debug_instr,
  uint64_t commits_info_4_debug_pc,
  uint64_t commits_info_4_traceBlockInPipe_ilastsize,
  uint64_t commits_info_4_traceBlockInPipe_iretire,
  uint64_t commits_info_4_traceBlockInPipe_itype,
  uint64_t commits_info_4_needFlush,
  uint64_t commits_info_4_isMove,
  uint64_t commits_info_4_loadWaitBit,
  uint64_t commits_info_4_rfWen,
  uint64_t commits_info_4_fpWen,
  uint64_t commits_info_4_instrSize,
  uint64_t commits_info_4_ftqOffset,
  uint64_t commits_info_4_ftqIdx_value,
  uint64_t commits_info_4_ftqIdx_flag,
  uint64_t commits_info_4_commitType,
  uint64_t commits_info_4_isVls,
  uint64_t commits_info_4_isHls,
  uint64_t commits_info_4_isVset,
  uint64_t commits_info_4_isRVC,
  uint64_t commits_info_4_vxsat,
  uint64_t commits_info_4_fflags,
  uint64_t commits_info_4_wflags,
  uint64_t commits_info_4_interrupt_safe,
  uint64_t commits_info_4_realDestSize,
  uint64_t commits_info_4_commit_w,
  uint64_t commits_info_4_commit_v,
  uint64_t commits_info_4_walk_v,
  uint64_t commits_info_5_dirtyVs,
  uint64_t commits_info_5_dirtyFs,
  uint64_t commits_info_5_debug_fuType,
  uint64_t commits_info_5_debug_pdest,
  uint64_t commits_info_5_debug_ldest,
  uint64_t commits_info_5_debug_instr,
  uint64_t commits_info_5_debug_pc,
  uint64_t commits_info_5_traceBlockInPipe_ilastsize,
  uint64_t commits_info_5_traceBlockInPipe_iretire,
  uint64_t commits_info_5_traceBlockInPipe_itype,
  uint64_t commits_info_5_needFlush,
  uint64_t commits_info_5_isMove,
  uint64_t commits_info_5_loadWaitBit,
  uint64_t commits_info_5_rfWen,
  uint64_t commits_info_5_fpWen,
  uint64_t commits_info_5_instrSize,
  uint64_t commits_info_5_ftqOffset,
  uint64_t commits_info_5_ftqIdx_value,
  uint64_t commits_info_5_ftqIdx_flag,
  uint64_t commits_info_5_commitType,
  uint64_t commits_info_5_isVls,
  uint64_t commits_info_5_isHls,
  uint64_t commits_info_5_isVset,
  uint64_t commits_info_5_isRVC,
  uint64_t commits_info_5_vxsat,
  uint64_t commits_info_5_fflags,
  uint64_t commits_info_5_wflags,
  uint64_t commits_info_5_interrupt_safe,
  uint64_t commits_info_5_realDestSize,
  uint64_t commits_info_5_commit_w,
  uint64_t commits_info_5_commit_v,
  uint64_t commits_info_5_walk_v,
  uint64_t commits_info_6_dirtyVs,
  uint64_t commits_info_6_dirtyFs,
  uint64_t commits_info_6_debug_fuType,
  uint64_t commits_info_6_debug_pdest,
  uint64_t commits_info_6_debug_ldest,
  uint64_t commits_info_6_debug_instr,
  uint64_t commits_info_6_debug_pc,
  uint64_t commits_info_6_traceBlockInPipe_ilastsize,
  uint64_t commits_info_6_traceBlockInPipe_iretire,
  uint64_t commits_info_6_traceBlockInPipe_itype,
  uint64_t commits_info_6_needFlush,
  uint64_t commits_info_6_isMove,
  uint64_t commits_info_6_loadWaitBit,
  uint64_t commits_info_6_rfWen,
  uint64_t commits_info_6_fpWen,
  uint64_t commits_info_6_instrSize,
  uint64_t commits_info_6_ftqOffset,
  uint64_t commits_info_6_ftqIdx_value,
  uint64_t commits_info_6_ftqIdx_flag,
  uint64_t commits_info_6_commitType,
  uint64_t commits_info_6_isVls,
  uint64_t commits_info_6_isHls,
  uint64_t commits_info_6_isVset,
  uint64_t commits_info_6_isRVC,
  uint64_t commits_info_6_vxsat,
  uint64_t commits_info_6_fflags,
  uint64_t commits_info_6_wflags,
  uint64_t commits_info_6_interrupt_safe,
  uint64_t commits_info_6_realDestSize,
  uint64_t commits_info_6_commit_w,
  uint64_t commits_info_6_commit_v,
  uint64_t commits_info_6_walk_v,
  uint64_t commits_info_7_dirtyVs,
  uint64_t commits_info_7_dirtyFs,
  uint64_t commits_info_7_debug_fuType,
  uint64_t commits_info_7_debug_pdest,
  uint64_t commits_info_7_debug_ldest,
  uint64_t commits_info_7_debug_instr,
  uint64_t commits_info_7_debug_pc,
  uint64_t commits_info_7_traceBlockInPipe_ilastsize,
  uint64_t commits_info_7_traceBlockInPipe_iretire,
  uint64_t commits_info_7_traceBlockInPipe_itype,
  uint64_t commits_info_7_needFlush,
  uint64_t commits_info_7_isMove,
  uint64_t commits_info_7_loadWaitBit,
  uint64_t commits_info_7_rfWen,
  uint64_t commits_info_7_fpWen,
  uint64_t commits_info_7_instrSize,
  uint64_t commits_info_7_ftqOffset,
  uint64_t commits_info_7_ftqIdx_value,
  uint64_t commits_info_7_ftqIdx_flag,
  uint64_t commits_info_7_commitType,
  uint64_t commits_info_7_isVls,
  uint64_t commits_info_7_isHls,
  uint64_t commits_info_7_isVset,
  uint64_t commits_info_7_isRVC,
  uint64_t commits_info_7_vxsat,
  uint64_t commits_info_7_fflags,
  uint64_t commits_info_7_wflags,
  uint64_t commits_info_7_interrupt_safe,
  uint64_t commits_info_7_realDestSize,
  uint64_t commits_info_7_commit_w,
  uint64_t commits_info_7_commit_v,
  uint64_t commits_info_7_walk_v,
  uint64_t commits_walkValid_0,
  uint64_t commits_walkValid_1,
  uint64_t commits_walkValid_2,
  uint64_t commits_walkValid_3,
  uint64_t commits_walkValid_4,
  uint64_t commits_walkValid_5,
  uint64_t commits_walkValid_6,
  uint64_t commits_walkValid_7,
  uint64_t commits_isWalk,
  uint64_t commits_commitValid_0,
  uint64_t commits_commitValid_1,
  uint64_t commits_commitValid_2,
  uint64_t commits_commitValid_3,
  uint64_t commits_commitValid_4,
  uint64_t commits_commitValid_5,
  uint64_t commits_commitValid_6,
  uint64_t commits_commitValid_7,
  uint64_t commits_isCommit,
  uint64_t state,
  uint64_t stamp,
  char *site
) {}



void init_db(bool en, bool select_enable, const char *select_db){
  dump = en;
  if(!en) return;
  rc = sqlite3_open(":memory:", &mem_db);
  if(rc) {
    printf("Can't open database: %s\n", sqlite3_errmsg(mem_db));
    exit(0);
  } else {
    printf("Open database successfully\n");
  }

  if (select_enable) {
    char *select_db_list[256];
    int select_db_num = 0;

    const char *split_word = " ";
    char select_db_copy[1024];
    select_db_copy[0] = '\0';
    strncpy(select_db_copy, select_db, 1024);
    select_db_copy[1023] = '\0';
    char *str_p = strtok(select_db_copy, split_word);
    while (str_p) {
      select_db_list[select_db_num] = str_p;
      select_db_num ++;
      str_p = strtok(NULL, split_word);
    }
    enable_dump_L2MP = false;
    enable_dump_LoadAccessDB0 = false;
    enable_dump_TLLog = false;
    enable_dump_L2TlbMissQueue_hart0 = false;
    enable_dump_LoadPfTrace0_hart0 = false;
    enable_dump_LoadMissDB0 = false;
    enable_dump_LoadPfTrace1_hart0 = false;
    enable_dump_L1MissQMissTrace_hart0 = false;
    enable_dump_LoadTrace0_hart0 = false;
    enable_dump_BankConflict0 = false;
    enable_dump_LoadTraceMiss2_hart0 = false;
    enable_dump_InstTable0 = false;
    enable_dump_Tip_0 = false;
    enable_dump_PageCache_hart0 = false;
    enable_dump_FTQTable0 = false;
    enable_dump_LoadTrace1_hart0 = false;
    enable_dump_IfuWbToFtq0 = false;
    enable_dump_PTW_hart0 = false;
    enable_dump_LoadPfMshr1_hart0 = false;
    enable_dump_ICacheSRAMTable0 = false;
    enable_dump_LoadTraceMiss0_hart0 = false;
    enable_dump_LoadPfTrace2_hart0 = false;
    enable_dump_L1Tlb_hart0 = false;
    enable_dump_LoadTrace2_hart0 = false;
    enable_dump_LoadPfMshr0_hart0 = false;
    enable_dump_L2TlbPrefetch_hart0 = false;
    enable_dump_FetchToIBuffer0 = false;
    enable_dump_LoadPfMshr2_hart0 = false;
    enable_dump_LoadTraceMiss1_hart0 = false;

    {
      const char *table_name = "LoadPfTrace1_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfTrace1_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTraceMiss1_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTraceMiss1_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "FetchToIBuffer0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_FetchToIBuffer0 = true;
        break;
      }
    }


    {
      const char *table_name = "FTQTable0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_FTQTable0 = true;
        break;
      }
    }


    {
      const char *table_name = "PTW_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_PTW_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "L1Tlb_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_L1Tlb_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTraceMiss0_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTraceMiss0_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "ICacheSRAMTable0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_ICacheSRAMTable0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadPfMshr2_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfMshr2_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "L2MP";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_L2MP = true;
        break;
      }
    }


    {
      const char *table_name = "LoadPfMshr0_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfMshr0_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "L2TlbMissQueue_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_L2TlbMissQueue_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadPfMshr1_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfMshr1_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "InstTable0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_InstTable0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTrace0_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTrace0_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "L2TlbPrefetch_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_L2TlbPrefetch_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTrace2_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTrace2_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadPfTrace2_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfTrace2_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "Tip_0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_Tip_0 = true;
        break;
      }
    }


    {
      const char *table_name = "PageCache_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_PageCache_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTraceMiss2_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTraceMiss2_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadMissDB0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadMissDB0 = true;
        break;
      }
    }


    {
      const char *table_name = "BankConflict0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_BankConflict0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadAccessDB0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadAccessDB0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadTrace1_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadTrace1_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "L1MissQMissTrace_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_L1MissQMissTrace_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "IfuWbToFtq0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_IfuWbToFtq0 = true;
        break;
      }
    }


    {
      const char *table_name = "LoadPfTrace0_hart0";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_LoadPfTrace0_hart0 = true;
        break;
      }
    }


    {
      const char *table_name = "TLLog";
      for (int idx = 0; idx < select_db_num; idx++) {
        char *str_p = select_db_list[idx];
        int s_idx = 0;
        bool match = true;
        for (; (str_p[s_idx] != '\0') && (table_name[s_idx] != '\0'); s_idx ++) {
          if (str_p[s_idx] != table_name[s_idx]) {
            match = false;
            break;
          }
        }
        if (!match || (str_p[s_idx] != '\0'))  continue;

        enable_dump_TLLog = true;
        break;
      }
    }

  }

  init_db_LoadPfTrace2_hart0();
  init_db_LoadTrace0_hart0();
  init_db_PageCache_hart0();
  init_db_L1Tlb_hart0();
  init_db_LoadTrace2_hart0();
  init_db_L1MissQMissTrace_hart0();
  init_db_LoadPfTrace1_hart0();
  init_db_LoadMissDB0();
  init_db_InstTable0();
  init_db_LoadPfMshr0_hart0();
  init_db_L2MP();
  init_db_LoadTrace1_hart0();
  init_db_ICacheSRAMTable0();
  init_db_PTW_hart0();
  init_db_L2TlbPrefetch_hart0();
  init_db_FetchToIBuffer0();
  init_db_Tip_0();
  init_db_FTQTable0();
  init_db_LoadTraceMiss0_hart0();
  init_db_LoadTraceMiss1_hart0();
  init_db_L2TlbMissQueue_hart0();
  init_db_BankConflict0();
  init_db_IfuWbToFtq0();
  init_db_LoadPfMshr1_hart0();
  init_db_TLLog();
  init_db_LoadPfTrace0_hart0();
  init_db_LoadPfMshr2_hart0();
  init_db_LoadTraceMiss2_hart0();
  init_db_LoadAccessDB0();

}

void save_db(const char *zFilename) {
  printf("saving memdb to %s ...\n", zFilename);
  sqlite3 *disk_db;
  sqlite3_backup *pBackup;
  rc = sqlite3_open(zFilename, &disk_db);
  if(rc == SQLITE_OK){
    pBackup = sqlite3_backup_init(disk_db, "main", mem_db, "main");
    if(pBackup){
      (void)sqlite3_backup_step(pBackup, -1);
      (void)sqlite3_backup_finish(pBackup);
    }
    rc = sqlite3_errcode(disk_db);
  }
  sqlite3_close(disk_db);
}


