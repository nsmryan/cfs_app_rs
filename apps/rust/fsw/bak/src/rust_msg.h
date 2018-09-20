/*=======================================================================================
** File Name:  rust_msg.h
**
** Title:  Message Definition Header File for RUST Application
**
** $Author:    Noah Ryan
** $Revision: 1.1 $
** $Date:      2018-09-16
**
** Purpose:  To define RUST's command and telemetry message defintions 
**
** Modification History:
**   Date | Author | Description
**   ---------------------------
**   2018-09-16 | Noah Ryan | Build #: Code Started
**
**=====================================================================================*/
    
#ifndef _RUST_MSG_H_
#define _RUST_MSG_H_

/*
** Pragmas
*/

/*
** Include Files
*/



/*
** Local Defines
*/

/*
** RUST command codes
*/
#define RUST_NOOP_CC                 0
#define RUST_RESET_CC                1

/*
** Local Structure Declarations
*/
typedef struct
{
    uint8              TlmHeader[CFE_SB_TLM_HDR_SIZE];
    uint8              usCmdCnt;
    uint8              usCmdErrCnt;

    /* TODO:  Add declarations for additional housekeeping data here */

} RUST_HkTlm_t;


#endif /* _RUST_MSG_H_ */

/*=======================================================================================
** End of file rust_msg.h
**=====================================================================================*/
    