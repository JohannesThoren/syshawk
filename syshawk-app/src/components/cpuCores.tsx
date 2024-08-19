import { GoCpu } from "react-icons/go";
import { Cpu } from "../../types/system";
import { HiCube, HiCubeTransparent } from "react-icons/hi2";
import { HistoryRowReturnData } from "../../types/sysinfo";

import {
  CartesianGrid,
  Cell,
  Legend,
  Line,
  LineChart,
  Pie,
  PieChart,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from "recharts";
import { useMemo, useState } from "react";
import { MdKeyboardArrowDown } from "react-icons/md";
import { Collapse } from "react-collapse";
import objectHash from "object-hash";

function formatCoreValue(num: number) {
  return parseFloat(num.toPrecision(3));
}

export default function CpuCores({
  cpu,
  history,
}: {
  cpu: Cpu;
  history: HistoryRowReturnData[];
}) {
  const getData = (num: number) => {
    const normal = formatCoreValue(num);
    return [
      { name: "Used", value: normal },
      { name: "Free", value: 100 - normal },
    ];
  };

  return (
    <div
      className={
        "relative overflow-hidden p-4 bg-emerald-300/10 rounded-xl outline outline-1 outline-emerald-600/20 flex flex-col items-start gap-2"
      }
    >
      <div
        className={
          "absolute top-0 right-0 py-2 px-4 bg-emerald-800/20 rounded-bl-xl"
        }
      >
        {cpu.usage.toPrecision(3)}%
      </div>
      <h2
        className={
          "flex gap-2 text-emerald-900 items-center text-lg font-semibold"
        }
      >
        <GoCpu />
        {cpu.name}
      </h2>
      <div className={"flex flex-wrap gap-2"}>
        <div
          className={
            "rounded-full py-1 px-3 bg-emerald-950/20 flex gap-2 items-center"
          }
        >
          <HiCubeTransparent title="All cores" /> {cpu.cores} |{" "}
          <HiCube title="Physical cores" /> {cpu.physical_cores}
        </div>
        <div className={"rounded-full py-1 px-3 bg-emerald-600/20"}>
          {(cpu.speed / 1000).toPrecision(3)} GHz
        </div>
      </div>
      <div className={"grid grid-cols-2 md:grid-cols-4 w-full"}>
        {cpu.per_core_usage.map((value, i) => {
          const data = getData(value);
          return (
            <div key={`core-${i}`} className={"relative"}>
              <ResponsiveContainer width="100%" height={100}>
                <PieChart>
                  <Pie
                    key={`core-${i}`}
                    data={data}
                    dataKey="value"
                    nameKey="name"
                    innerRadius={30}
                    outerRadius={40}
                    stroke="none"
                  >
                    {data.map((entry, index) => (
                      <Cell
                        key={`cell-${index}`}
                        fill={entry.name === "Used" ? "#047857" : "#cccccc50"}
                      />
                    ))}
                  </Pie>
                </PieChart>
              </ResponsiveContainer>
              <p
                className={
                  "text-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2"
                }
              >
                {value.toPrecision(3)}%
              </p>
            </div>
          );
        })}
      </div>

      <CpuHistory history={history} />
    </div>
  );
}

function CpuHistory({ history }: { history: HistoryRowReturnData[] }) {
  const [cpuCount, setCpuCount] = useState(0);
  const [open, setOpen] = useState(false);

  const cpuNames = useMemo(() => {
    return Array.from({ length: cpuCount }).map((_, i) => `CPU ${i + 1}`);
  }, [cpuCount]);

  const cpuColors = useMemo(
    () =>
      Object.fromEntries(
        cpuNames.map((name) => [name, `#${objectHash(name).slice(1, 7)}`])
      ),
    [cpuCount]
  );

  const data = useMemo(() => {
    if (history.length === 0) return [];
    const foundCpuCount =
      history[0].system_info?.cpu.per_core_usage.length || 0;
    if (foundCpuCount !== cpuCount) {
      setCpuCount(foundCpuCount);
    }

    return history.map((row) => {
      const perCoreUsage = row.system_info?.cpu.per_core_usage || [];
      const data = perCoreUsage.reduce(
        (acc: any, usage, index) => {
          acc[cpuNames[index]] = formatCoreValue(usage || 0);
          return acc;
        },
        { name: row.time_stamp }
      );

      return data;
    });
  }, [history]);

  return (
    <div className={"w-full flex flex-col gap-2 mt-5"}>
      <button
        className="text-emerald-900 font-semibold bg-emerald-300/20 py-2 px-4 rounded-xl transition-colors hover:bg-emerald-300/40"
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
              <CartesianGrid strokeDasharray="4 4" />
              <XAxis
                dataKey="name"
                tickFormatter={(value) => new Date(value).toLocaleTimeString()}
              />
              <YAxis tickFormatter={(value) => `${value}%`} />
              <Tooltip />
              {Array.from({ length: cpuCount }).map((_, i) => (
                <Line
                  key={cpuNames[i]}
                  type="monotone"
                  dataKey={cpuNames[i]}
                  stroke={cpuColors[cpuNames[i]]}
                  strokeWidth={2}
                />
              ))}
            </LineChart>
          </ResponsiveContainer>
        </div>
      </Collapse>
    </div>
  );
}
