import SysInfo from "../lib/sysinfo";
import { useParams, useNavigate } from "react-router-dom";
import CpuCores from "../components/cpuCores";
import MemoryDisplay from "../components/memory";
import ProbeInfo from "../components/probeInfo";
import FsDisply from "../components/FsDisplay";

export default function Single() {
  const { id } = useParams();
  const navigate = useNavigate();

  if (!id) {
    navigate("/");
    return "Not found";
  }

  const sysinfo = SysInfo.useSysinfoHistory(id, 20);

  if (sysinfo.isLoading) {
    return "Loading...";
  }

  if (!sysinfo.data) {
    return "No data";
  }

  const single = sysinfo.data[0];

  return (
    <div className={"p-5 space-y-4 max-w-7xl mx-auto w-full mb-[300px]"}>
      <ProbeInfo data={single} />
      <div className="grid gap-4">
      {single.system_info?.cpu && (
        <CpuCores cpu={single.system_info?.cpu} history={sysinfo.data} />
      )}
      {single.system_info?.memory &&(
        <MemoryDisplay memory={single.system_info?.memory} history={sysinfo.data} />
      )}
      {single.system_info?.fs && (
        <FsDisply fs={single.system_info?.fs} />
      )}
      </div>
    </div>
  );
}
