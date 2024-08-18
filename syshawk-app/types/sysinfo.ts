import {System} from "./system";

export type HistoryRowReturnData = {
    probe_id: string;
    system_info?: System;
    time_stamp: Date;
    status_code: number;
}