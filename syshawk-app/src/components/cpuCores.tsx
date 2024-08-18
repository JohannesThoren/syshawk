import { GoCpu } from "react-icons/go";
import { Cpu } from "../../types/system";
import { HiCube, HiCubeTransparent } from "react-icons/hi2";
import { HistoryRowReturnData } from "../../types/sysinfo";

import { Cell, Pie, PieChart, ResponsiveContainer } from "recharts";
import { useMemo, useState } from "react";
import { MdKeyboardArrowDown } from "react-icons/md";
import { Collapse } from "react-collapse";

export default function CpuCores({
  cpu,
  history,
}: {
  cpu: Cpu;
  history: HistoryRowReturnData[];
}) {
  const getData = (num: number) => {
    const normal = parseFloat(num.toPrecision(3));
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
                        fill={entry.name === "Used" ? "#d88495" : "#cccccc50"}
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
  const [open, setOpen] = useState(false);
  const data = useMemo(() => {
    const cores = history.map(row => row.system_info?.cpu.per_core_usage).flat();
    
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
      {/* <Collapse isOpened={open}>
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
      */}
    </div>
  );
}
