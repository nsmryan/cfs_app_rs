/*=======================================================================================
** File Name:  rust_private_ids.h
**
** Title:  ID Header File for RUST Application
**
** $Author:    Noah Ryan
** $Revision: 1.1 $
** $Date:      2018-09-16
**
** Purpose:  This header file contains declarations and definitions of RUST's private IDs.
**
** Modification History:
**   Date | Author | Description
**   ---------------------------
**   2018-09-16 | Noah Ryan | Build #: Code Started
**
**=====================================================================================*/
    
#ifndef _RUST_PRIVATE_IDS_H_
#define _RUST_PRIVATE_IDS_H_

/*
** Pragmas
*/

/*
** Include Files
*/

/*
** Local Defines
*/

/* Event IDs */
#define RUST_RESERVED_EID  0

#define RUST_INF_EID        1
#define RUST_INIT_INF_EID   2
#define RUST_ILOAD_INF_EID  3
#define RUST_CDS_INF_EID    4
#define RUST_CMD_INF_EID    5

#define RUST_ERR_EID         51
#define RUST_INIT_ERR_EID    52
#define RUST_ILOAD_ERR_EID   53
#define RUST_CDS_ERR_EID     54
#define RUST_CMD_ERR_EID     55
#define RUST_PIPE_ERR_EID    56
#define RUST_MSGID_ERR_EID   57
#define RUST_MSGLEN_ERR_EID  58

#define RUST_EVT_CNT  14

/*
** Local Structure Declarations
*/

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

#endif /* _RUST_PRIVATE_IDS_H_ */

/*=======================================================================================
** End of file rust_private_ids.h
**=====================================================================================*/
    