import SysInfo from "../lib/sysinfo";

export default function Home() {
    const sysinfo = SysInfo.useAllSysinfo()

    return <>
        <h1 className={"text-lg"}>
            home
        </h1>
        <pre>
            {JSON.stringify(sysinfo.data, null, 2)}
        </pre>
    </>
}