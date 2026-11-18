
#include <map>
#include <string>
#include <stdint.h>
using namespace std;

map<string, uint64_t> constantinMap;

void constantinInit(){
  constantinMap["isWriteICacheSRAMTable0"] = 0;
  constantinMap["hyuPriorityAddCtr"] = 2;
  constantinMap["StoreBufferThreshold_0"] = 7;
  constantinMap["isWriteFetchToIBufferTable0"] = 0;
  constantinMap["EnableHybridUnitReplay"] = 1;
  constantinMap["isWritePTWTable0"] = 0;
  constantinMap["isWriteBankConflictTable0"] = 0;
  constantinMap["CorrectMissTrain0"] = 0;
  constantinMap["isWriteL1MissQMissTable0"] = 0;
  constantinMap["nMaxPrefetchEntry0"] = 14;
  constantinMap["isWriteL2TlbMissQueueTable0"] = 0;
  constantinMap["EnableMdp"] = 1;
  constantinMap["enableDynamicPrefetcher0"] = 1;
  constantinMap["isFirstHitWrite0"] = 0;
  constantinMap["hyuPrioritySubCtr"] = 5;
  constantinMap["isWritePageCacheTable0"] = 0;
  constantinMap["StoreBufferBase_0"] = 4;
  constantinMap["depth0"] = 32;
  constantinMap["isWriteLoadAccessTable0"] = 0;
  constantinMap["ColdDownThreshold_0"] = 12;
  constantinMap["isWriteL2TlbPrefetchTable0"] = 0;
  constantinMap["ForceWriteUpper_0"] = 60;
  constantinMap["enableLoadBalance"] = 1;
  constantinMap["enableStoreBalance"] = 1;
  constantinMap["isWriteL1TlbTable0"] = 0;
  constantinMap["isWriteLoadMissTable0"] = 0;
  constantinMap["isWriteFTQTable0"] = 0;
  constantinMap["ForceWriteLower_0"] = 55;
  constantinMap["LFSTEnable"] = 1;
  constantinMap["isWriteIfuWbToFtqTable0"] = 0;
  constantinMap["StoreWaitThreshold_0"] = 0;

}

#include <iostream>
#include <fstream>
#include <map>
#include <string>
#include <cstdlib>
#include <stdint.h>
using namespace std;

fstream cf;
extern map<string, uint64_t> constantinMap;

void constantinLoad() {
  constantinInit();
  uint64_t num;
  const char *noop_home = getenv("NOOP_HOME");
#ifdef NOOP_HOME
  if (!noop_home) {
    noop_home = NOOP_HOME;
  }
#endif
  string noop_home_s = noop_home;
  string tmp = noop_home_s + "/build/constantin.txt";
  cf.open(tmp.c_str(), ios::in);
  if(cf.good()){
    while (cf >> tmp >> num) {
      constantinMap[tmp] = num;
    }
  }else{
    cout << "[WARNING] " << tmp << " does not exist, so all constants default to initialized values." << endl;
  }
  cf.close();

}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteICacheSRAMTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteICacheSRAMTable0"]){
    cout << "isWriteICacheSRAMTable0" << " = " << constantinMap["isWriteICacheSRAMTable0"] << endl;
    a = constantinMap["isWriteICacheSRAMTable0"];
  }
  return constantinMap["isWriteICacheSRAMTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t hyuPriorityAddCtr_constantin_read() {
  static int a = -1;
  if(a != constantinMap["hyuPriorityAddCtr"]){
    cout << "hyuPriorityAddCtr" << " = " << constantinMap["hyuPriorityAddCtr"] << endl;
    a = constantinMap["hyuPriorityAddCtr"];
  }
  return constantinMap["hyuPriorityAddCtr"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t StoreBufferThreshold_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["StoreBufferThreshold_0"]){
    cout << "StoreBufferThreshold_0" << " = " << constantinMap["StoreBufferThreshold_0"] << endl;
    a = constantinMap["StoreBufferThreshold_0"];
  }
  return constantinMap["StoreBufferThreshold_0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteFetchToIBufferTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteFetchToIBufferTable0"]){
    cout << "isWriteFetchToIBufferTable0" << " = " << constantinMap["isWriteFetchToIBufferTable0"] << endl;
    a = constantinMap["isWriteFetchToIBufferTable0"];
  }
  return constantinMap["isWriteFetchToIBufferTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t EnableHybridUnitReplay_constantin_read() {
  static int a = -1;
  if(a != constantinMap["EnableHybridUnitReplay"]){
    cout << "EnableHybridUnitReplay" << " = " << constantinMap["EnableHybridUnitReplay"] << endl;
    a = constantinMap["EnableHybridUnitReplay"];
  }
  return constantinMap["EnableHybridUnitReplay"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWritePTWTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWritePTWTable0"]){
    cout << "isWritePTWTable0" << " = " << constantinMap["isWritePTWTable0"] << endl;
    a = constantinMap["isWritePTWTable0"];
  }
  return constantinMap["isWritePTWTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteBankConflictTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteBankConflictTable0"]){
    cout << "isWriteBankConflictTable0" << " = " << constantinMap["isWriteBankConflictTable0"] << endl;
    a = constantinMap["isWriteBankConflictTable0"];
  }
  return constantinMap["isWriteBankConflictTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t CorrectMissTrain0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["CorrectMissTrain0"]){
    cout << "CorrectMissTrain0" << " = " << constantinMap["CorrectMissTrain0"] << endl;
    a = constantinMap["CorrectMissTrain0"];
  }
  return constantinMap["CorrectMissTrain0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteL1MissQMissTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteL1MissQMissTable0"]){
    cout << "isWriteL1MissQMissTable0" << " = " << constantinMap["isWriteL1MissQMissTable0"] << endl;
    a = constantinMap["isWriteL1MissQMissTable0"];
  }
  return constantinMap["isWriteL1MissQMissTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t nMaxPrefetchEntry0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["nMaxPrefetchEntry0"]){
    cout << "nMaxPrefetchEntry0" << " = " << constantinMap["nMaxPrefetchEntry0"] << endl;
    a = constantinMap["nMaxPrefetchEntry0"];
  }
  return constantinMap["nMaxPrefetchEntry0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteL2TlbMissQueueTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteL2TlbMissQueueTable0"]){
    cout << "isWriteL2TlbMissQueueTable0" << " = " << constantinMap["isWriteL2TlbMissQueueTable0"] << endl;
    a = constantinMap["isWriteL2TlbMissQueueTable0"];
  }
  return constantinMap["isWriteL2TlbMissQueueTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t EnableMdp_constantin_read() {
  static int a = -1;
  if(a != constantinMap["EnableMdp"]){
    cout << "EnableMdp" << " = " << constantinMap["EnableMdp"] << endl;
    a = constantinMap["EnableMdp"];
  }
  return constantinMap["EnableMdp"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t enableDynamicPrefetcher0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["enableDynamicPrefetcher0"]){
    cout << "enableDynamicPrefetcher0" << " = " << constantinMap["enableDynamicPrefetcher0"] << endl;
    a = constantinMap["enableDynamicPrefetcher0"];
  }
  return constantinMap["enableDynamicPrefetcher0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isFirstHitWrite0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isFirstHitWrite0"]){
    cout << "isFirstHitWrite0" << " = " << constantinMap["isFirstHitWrite0"] << endl;
    a = constantinMap["isFirstHitWrite0"];
  }
  return constantinMap["isFirstHitWrite0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t hyuPrioritySubCtr_constantin_read() {
  static int a = -1;
  if(a != constantinMap["hyuPrioritySubCtr"]){
    cout << "hyuPrioritySubCtr" << " = " << constantinMap["hyuPrioritySubCtr"] << endl;
    a = constantinMap["hyuPrioritySubCtr"];
  }
  return constantinMap["hyuPrioritySubCtr"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWritePageCacheTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWritePageCacheTable0"]){
    cout << "isWritePageCacheTable0" << " = " << constantinMap["isWritePageCacheTable0"] << endl;
    a = constantinMap["isWritePageCacheTable0"];
  }
  return constantinMap["isWritePageCacheTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t StoreBufferBase_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["StoreBufferBase_0"]){
    cout << "StoreBufferBase_0" << " = " << constantinMap["StoreBufferBase_0"] << endl;
    a = constantinMap["StoreBufferBase_0"];
  }
  return constantinMap["StoreBufferBase_0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t depth0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["depth0"]){
    cout << "depth0" << " = " << constantinMap["depth0"] << endl;
    a = constantinMap["depth0"];
  }
  return constantinMap["depth0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteLoadAccessTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteLoadAccessTable0"]){
    cout << "isWriteLoadAccessTable0" << " = " << constantinMap["isWriteLoadAccessTable0"] << endl;
    a = constantinMap["isWriteLoadAccessTable0"];
  }
  return constantinMap["isWriteLoadAccessTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t ColdDownThreshold_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["ColdDownThreshold_0"]){
    cout << "ColdDownThreshold_0" << " = " << constantinMap["ColdDownThreshold_0"] << endl;
    a = constantinMap["ColdDownThreshold_0"];
  }
  return constantinMap["ColdDownThreshold_0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteL2TlbPrefetchTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteL2TlbPrefetchTable0"]){
    cout << "isWriteL2TlbPrefetchTable0" << " = " << constantinMap["isWriteL2TlbPrefetchTable0"] << endl;
    a = constantinMap["isWriteL2TlbPrefetchTable0"];
  }
  return constantinMap["isWriteL2TlbPrefetchTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t ForceWriteUpper_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["ForceWriteUpper_0"]){
    cout << "ForceWriteUpper_0" << " = " << constantinMap["ForceWriteUpper_0"] << endl;
    a = constantinMap["ForceWriteUpper_0"];
  }
  return constantinMap["ForceWriteUpper_0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t enableLoadBalance_constantin_read() {
  static int a = -1;
  if(a != constantinMap["enableLoadBalance"]){
    cout << "enableLoadBalance" << " = " << constantinMap["enableLoadBalance"] << endl;
    a = constantinMap["enableLoadBalance"];
  }
  return constantinMap["enableLoadBalance"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t enableStoreBalance_constantin_read() {
  static int a = -1;
  if(a != constantinMap["enableStoreBalance"]){
    cout << "enableStoreBalance" << " = " << constantinMap["enableStoreBalance"] << endl;
    a = constantinMap["enableStoreBalance"];
  }
  return constantinMap["enableStoreBalance"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteL1TlbTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteL1TlbTable0"]){
    cout << "isWriteL1TlbTable0" << " = " << constantinMap["isWriteL1TlbTable0"] << endl;
    a = constantinMap["isWriteL1TlbTable0"];
  }
  return constantinMap["isWriteL1TlbTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteLoadMissTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteLoadMissTable0"]){
    cout << "isWriteLoadMissTable0" << " = " << constantinMap["isWriteLoadMissTable0"] << endl;
    a = constantinMap["isWriteLoadMissTable0"];
  }
  return constantinMap["isWriteLoadMissTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteFTQTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteFTQTable0"]){
    cout << "isWriteFTQTable0" << " = " << constantinMap["isWriteFTQTable0"] << endl;
    a = constantinMap["isWriteFTQTable0"];
  }
  return constantinMap["isWriteFTQTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t ForceWriteLower_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["ForceWriteLower_0"]){
    cout << "ForceWriteLower_0" << " = " << constantinMap["ForceWriteLower_0"] << endl;
    a = constantinMap["ForceWriteLower_0"];
  }
  return constantinMap["ForceWriteLower_0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t LFSTEnable_constantin_read() {
  static int a = -1;
  if(a != constantinMap["LFSTEnable"]){
    cout << "LFSTEnable" << " = " << constantinMap["LFSTEnable"] << endl;
    a = constantinMap["LFSTEnable"];
  }
  return constantinMap["LFSTEnable"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t isWriteIfuWbToFtqTable0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["isWriteIfuWbToFtqTable0"]){
    cout << "isWriteIfuWbToFtqTable0" << " = " << constantinMap["isWriteIfuWbToFtqTable0"] << endl;
    a = constantinMap["isWriteIfuWbToFtqTable0"];
  }
  return constantinMap["isWriteIfuWbToFtqTable0"];
}

#include <map>
#include <string>
#include <stdint.h>
#include <assert.h>
using namespace std;
extern map<string, uint64_t> constantinMap;
extern "C" uint64_t StoreWaitThreshold_0_constantin_read() {
  static int a = -1;
  if(a != constantinMap["StoreWaitThreshold_0"]){
    cout << "StoreWaitThreshold_0" << " = " << constantinMap["StoreWaitThreshold_0"] << endl;
    a = constantinMap["StoreWaitThreshold_0"];
  }
  return constantinMap["StoreWaitThreshold_0"];
}
