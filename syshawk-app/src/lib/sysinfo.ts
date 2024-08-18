import { useQuery } from "@tanstack/react-query";
import axios from "axios";
import { BASE_URL } from "../shared/env.ts";
import { HistoryRowReturnData } from "../../types/sysinfo.ts";

export default class SysInfo {
    public static readonly sysInfoRefetchInterval = 5 * 1000;

  static useAllSysinfo() {
    return useQuery({
      queryKey: ["sysinfo"],
      queryFn: () =>
        axios
          .get(`${BASE_URL}sysinfo`)
          .then((res) => res.data as HistoryRowReturnData[]),
          refetchInterval: SysInfo.sysInfoRefetchInterval
    });
  }

  static useSysinfo(id: string) {
    return useQuery({
      queryKey: ["sysinfo", id],
      queryFn: () =>
        axios
          .get(`${BASE_URL}sysinfo/${id}`)
          .then((res) => res.data as HistoryRowReturnData),
       refetchInterval: SysInfo.sysInfoRefetchInterval
    });
  }

  static useSysinfoHistory(id: string) {
    return useQuery({
      queryKey: ["sysinfo", id, "history"],
      queryFn: () =>
        axios
          .get(`${BASE_URL}sysinfo/${id}/history`)
          .then((res) => res.data as HistoryRowReturnData[]),
       refetchInterval: SysInfo.sysInfoRefetchInterval
    });
  }
}
