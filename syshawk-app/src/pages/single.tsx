import { useMemo } from "react";
import SysInfo from "../lib/sysinfo";
import { useParams, useNavigate } from "react-router-dom";
import { PieChart, Pie, Cell, Tooltip, ResponsiveContainer } from "recharts";

export default function Single() {
  const { id } = useParams();
  const navigate = useNavigate();

  if (!id) {
    navigate("/");
    return "Not found";
  }

  const sysinfo = SysInfo.useSysinfo(id);

  if (sysinfo.isLoading) {
    return "Loading...";
  }

  if (!sysinfo.data) {
    return "No data";
  }

  return (
    <>
      <h1 className={"text-lg"}>Probe {sysinfo.data?.probe_id}</h1>
      <div className={"grid grid-cols-6"}>
        <CpuCores usage={sysinfo.data.system_info?.cpu.per_core_usage || []} />
      </div>
      <pre>{JSON.stringify(sysinfo.data, null, 2)}</pre>
    </>
  );
}

function CpuCores({ usage }: { usage: number[] }) {
  return (
    <div>
      <h2 className={"text-lg"}>Core {usage}</h2>
        <PieChart width={400} height={400}>
        <Tooltip />
          {usage.map((value, index) => (
            <Pie
              key={`cpu-${index}`}
              dataKey={`cpu-${index}`}
              data={usage}
              nameKey="cpu"
              cx="50%"
              cy="50%"
              innerRadius={40}
              outerRadius={80}
              fill="#8884d8"
              label
            >
              <Cell key={`cpu-${index}`} fill={`rgba(255, 255, 255, ${value})`} />
            </Pie>
          ))}
        </PieChart>
    </div>
  );
}
