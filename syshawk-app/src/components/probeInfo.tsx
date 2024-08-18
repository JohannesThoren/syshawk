import { useEffect, useState } from "react";
import { HistoryRowReturnData } from "../../types/sysinfo";
import SysInfo from "../lib/sysinfo";
import { MdFingerprint } from "react-icons/md";

function formatUptime(uptime: number) {
  const days = Math.floor(uptime / 86400);
  const hours = Math.floor((uptime % 86400) / 3600);
  const minutes = Math.floor(((uptime % 86400) % 3600) / 60);
  const seconds = Math.floor(((uptime % 86400) % 3600) % 60);
  return `${days}d ${hours}h ${minutes}m ${seconds}s`;
}

export default function ProbeInfo({ data }: { data: HistoryRowReturnData }) {
  const [refetchIn, setRefetchIn] = useState(SysInfo.sysInfoRefetchInterval);

  useEffect(() => {
    const interval = setInterval(() => {
      setRefetchIn((prev) => prev - 100);
    }, 100);
    return () => {
      clearInterval(interval);
      setRefetchIn(SysInfo.sysInfoRefetchInterval);
    };
  }, [data]);

  return (
    <div className={"grid gap-2"}>
      <h1 className="flex items-start text-2xl font-semibold border-b pb-1 border-slate-300/40">
        {data.system_info?.hostname}
        <span className={"text-sm text-slate-400 ml-2"}>
          # {data.probe_id}
        </span>
      </h1>
      <div className="flex flex-wrap gap-2">
        <div
          className={
            "flex items-center rounded-full py-1 px-2 bg-emerald-600/20"
          }
        >
          <div
            className={
              "w-[50px] h-5 bg-slate-300 rounded-full relative overflow-hidden"
            }
          >
            <div
              className={"absolute top-0 left-0 h-full bg-emerald-400"}
              style={{
                width: `${(refetchIn / SysInfo.sysInfoRefetchInterval) * 100}%`,
                transition: "width 0.1s",
              }}
            ></div>
          </div>
        </div>
        <div className={"rounded-full py-1 px-3 bg-emerald-600/20"}>
          up for {formatUptime(data.system_info?.uptime || 0)}
        </div>
      </div>
    </div>
  );
}
