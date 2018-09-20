/*=======================================================================================
** File Name:  rust_app.h
**
** Title:  Header File for RUST Application
**
** $Author:    Noah Ryan
** $Revision: 1.1 $
** $Date:      2018-09-16
**
** Purpose:  To define RUST's internal macros, data types, global variables and
**           function prototypes
**
** Modification History:
**   Date | Author | Description
**   ---------------------------
**   2018-09-16 | Noah Ryan | Build #: Code Started
**
**=====================================================================================*/
    
#ifndef _RUST_APP_H_
#define _RUST_APP_H_

/*
** Pragmas
*/

/*
** Include Files
*/
#include <errno.h>
#include <string.h>
#include <unistd.h>

#include "rust_platform_cfg.h"
#include "rust_mission_cfg.h"
#include "rust_private_ids.h"
#include "rust_private_types.h"
#include "rust_perfids.h"
#include "rust_msgids.h"
#include "rust_msg.h"



/*
** Local Defines
*/
#define RUST_TIMEOUT_MSEC    1000

/*
** Local Structure Declarations
*/
typedef struct
{
    /* CFE Event table */
    CFE_EVS_BinFilter_t  EventTbl[RUST_EVT_CNT];

    /* CFE scheduling pipe */
    CFE_SB_PipeId_t  SchPipeId; 
    uint16           usSchPipeDepth;
    char             cSchPipeName[OS_MAX_API_NAME];

    /* CFE command pipe */
    CFE_SB_PipeId_t  CmdPipeId;
    uint16           usCmdPipeDepth;
    char             cCmdPipeName[OS_MAX_API_NAME];
    
    /* CFE telemetry pipe */
    CFE_SB_PipeId_t  TlmPipeId;
    uint16           usTlmPipeDepth;
    char             cTlmPipeName[OS_MAX_API_NAME];

    /* Task-related */
    uint32  uiRunStatus;
    
    /* Input data - from I/O devices or subscribed from other apps' output data.
       Data structure should be defined in rust/fsw/src/rust_private_types.h */
    RUST_InData_t   InData;

    /* Output data - to be published at the end of a Wakeup cycle.
       Data structure should be defined in rust/fsw/src/rust_private_types.h */
    RUST_OutData_t  OutData;

    /* Housekeeping telemetry - for downlink only.
       Data structure should be defined in rust/fsw/src/rust_msg.h */
    RUST_HkTlm_t  HkTlm;

    /* TODO:  Add declarations for additional private data here */
} RUST_AppData_t;

/*
** External Global Variables
*/

/*
** Global Variables
*/

/*
** Local Variables
*/

/*
** Local Function Prototypes
*/
int32  RUST_InitApp(void);
int32  RUST_InitEvent(void);
int32  RUST_InitData(void);
int32  RUST_InitPipe(void);

void  RUST_AppMain(void);

void  RUST_CleanupCallback(void);

int32  RUST_RcvMsg(int32 iBlocking);

void  RUST_ProcessNewData(void);
void  RUST_ProcessNewCmds(void);
void  RUST_ProcessNewAppCmds(CFE_SB_Msg_t*);

void  RUST_ReportHousekeeping(void);
void  RUST_SendOutData(void);

boolean  RUST_VerifyCmdLength(CFE_SB_Msg_t*, uint16);

#endif /* _RUST_APP_H_ */

/*=======================================================================================
** End of file rust_app.h
**=====================================================================================*/
    