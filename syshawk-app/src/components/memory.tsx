import { FaMemory } from "react-icons/fa6";
import { Memory } from "../../types/system";
import {
  Bar,
  BarChart,
  Legend,
  Line,
  LineChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { HistoryRowReturnData } from "../../types/sysinfo";
import { useMemo, useState } from "react";
import { Collapse } from "react-collapse";
import { MdKeyboardArrowDown } from "react-icons/md";
const formatBytes = (bytes: number) => {
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  if (bytes === 0) return 0 + " " + sizes[0];
  const i = parseInt(Math.floor(Math.log(bytes) / Math.log(1024)).toString());
  if (i === 0) return bytes + " " + sizes[i];
  return (bytes / Math.pow(1024, i)).toFixed(1) + " " + sizes[i];
};

export default function MemoryDisplay({
  memory,
  history,
}: {
  memory: Memory;
  history: HistoryRowReturnData[];
}) {
  const data = [
    {
      name: "Memory",
      used: memory.used_memory,
      avalible: memory.available_memory,
    },
    {
      name: "Swap",
      used: memory.swap_used,
      avalible: memory.swap_total - memory.swap_used,
    },
  ];

  return (
    <div
      className={
        "overflow-hidden relative p-4 bg-amber-300/10 rounded-xl outline outline-1 outline-amber-600/20 flex flex-col items-start gap-2"
      }
    >
      <div
        className={
          "absolute top-0 right-0 py-2 px-4 bg-amber-800/20 rounded-bl-xl"
        }
      >
        {((memory.used_memory / memory.total_memory) * 100).toFixed(2)}%
      </div>
      <h2
        className={
          "flex gap-2 text-amber-900 items-center text-xl font-semibold"
        }
      >
        <FaMemory />
        Memory
      </h2>
      <div className="max-w-full w-full h-[150px]">
        <ResponsiveContainer width="100%" height={150}>
          <BarChart
            layout="vertical"
            data={data}
          >
            <Bar barSize={50} stackId="a" dataKey="used" fill="#f7c075" />
            <Bar barSize={50} stackId="a" dataKey="avalible" fill="#bae697" />
            <Tooltip formatter={formatBytes} />
            <XAxis type="number" hide />
            <YAxis type="category" dataKey="name" />
            <Legend verticalAlign="top" align="left" />
          </BarChart>
        </ResponsiveContainer>
      </div>
      <MemoryHistory history={history} />
    </div>
  );
}

function MemoryHistory({ history }: { history: HistoryRowReturnData[] }) {
  const [open, setOpen] = useState(false);
  const data = useMemo(() => history.map((row) => ({
    name: row.time_stamp,
    usedRam: row.system_info?.memory.used_memory,
    avalibleRam: row.system_info?.memory.available_memory,
    swapUsed: row.system_info?.memory.swap_used,
    swapTotal: row.system_info?.memory.swap_total,
  })).reverse(), [history]);

  return (
    <div className={"w-full flex flex-col gap-2 mt-5"}>
      <button
        className="text-amber-900 font-semibold bg-amber-300/20 py-2 px-4 rounded-xl transition-colors hover:bg-amber-300/40"
        onClick={() => setOpen(!open)}
      >
        History{" "}
        <MdKeyboardArrowDown
          className="inline transition-all"
          style={{ transform: open ? "rotate(180deg)" : "" }}
        />
      </button>
      <Collapse isOpened={open}>
        <div className="max-w-full w-full h-[500px]">
          <ResponsiveContainer width="100%" height={"100%"}>
            <LineChart width={500} height={400} data={data}>
              <Legend />
              <XAxis
                dataKey="name"
                tickFormatter={(value) => new Date(value).toLocaleTimeString()}
              />
              <YAxis tickFormatter={formatBytes} />
              <Tooltip formatter={formatBytes} />
              <Line strokeWidth={3} dataKey="usedRam" stroke="#FFC107" />
              <Line strokeWidth={3} dataKey="avalibleRam" stroke="#8BC34A" />
              <Line strokeWidth={3} dataKey="swapUsed" stroke="#03A9F4" />
              <Line strokeWidth={3} dataKey="swapTotal" stroke="#FF69B4" />
            </LineChart>
          </ResponsiveContainer>
        </div>
      </Collapse>
    </div>
  );
}
