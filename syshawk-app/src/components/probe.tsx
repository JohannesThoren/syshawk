import { Link } from "react-router-dom";
import { HistoryRowReturnData } from "../../types/sysinfo";

export default function ProbeItem({ data }: { data: HistoryRowReturnData }) {
  return (
    <Link
      to={`/system/${data.probe_id}`}
      className="p-4 rounded-xl bg-slate-600 hover:bg-slate-800 transition-colors text-slate-50"
    >
      <h1 className="flex items-start text-2xl font-semibold border-b pb-1 border-slate-300/40">
        {data.system_info?.hostname}
        <span className={"text-sm text-slate-400 ml-2"}># {data.probe_id}</span>
      </h1>
      {data.time_stamp && (
        <p className="text-slate-400">
          {new Date(data.time_stamp).toLocaleString()}
        </p>
      )}
    </Link>
  );
}
