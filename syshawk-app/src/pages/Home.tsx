import ProbeItem from "../components/probe";
import SysInfo from "../lib/sysinfo";

export default function Home() {
    const sysinfo = SysInfo.useAllSysinfo()

    return <div className="flex flex-col gap-3 p-5 md:p-10">
        <h1 className="text-3xl font-bold text-slate-700">
            Welcome to SysHawk
        </h1>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
            {sysinfo.data?.map((info) => <ProbeItem key={info.probe_id} data={info} />)}
        </div>
    </div>
}