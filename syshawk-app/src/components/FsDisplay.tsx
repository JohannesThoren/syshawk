import { HiSquare3Stack3D } from "react-icons/hi2";
import { Fs } from "../../types/system";
import { FiHardDrive } from "react-icons/fi";
import { BsUsbDrive } from "react-icons/bs";
import { formatBytes } from "../utils/format";

export default function FsDisplay({ fs }: { fs: Fs[] }) {
  return (
    <div
      className={
        "overflow-hidden relative p-4 bg-indigo-300/10 rounded-xl outline outline-1 outline-indigo-600/20 flex flex-col items-start gap-2"
      }
    >
      <h2
        className={
          "flex gap-2 text-indigo-900 items-center text-xl font-semibold"
        }
      >
        <HiSquare3Stack3D />
        File System
      </h2>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-4 w-full">
        {fs.map((drive) => (
          <Drive key={drive.drive_name + drive.mount_point} drive={drive} />
        ))}
      </div>
    </div>
  );
}

const getIcon = (isRemovable: boolean) => {
  if (isRemovable) {
    return <BsUsbDrive title="Removable" />;
  }
  return <FiHardDrive title="Fixed" />;
};

function Drive({ drive }: { drive: Fs }) {
  const usedSpace = `${((drive.used / drive.size) * 100).toFixed(2)}%`;

  return (
    <div className="relative pt-2 p-4 bg-indigo-700/10 rounded-xl space-y-1 overflow-hidden text-indigo-900">
      <div className="absolute top-0 right-0 px-3 py-1 bg-indigo-300/50 rounded-bl-xl">
        {drive.fs_type}
      </div>
      <h3 className="flex items-center gap-2 text-lg font-semibold">
        {getIcon(drive.is_removable)} {drive.drive_name || "No Name"}
      </h3>
      <p className="text-indigo-900 text-sm font-semibold">
        <span className="font-light">Mounted at:</span> {drive.mount_point}
      </p>
      <div className="h-3 rounded-full w-full bg-indigo-100/50">
        <div
          className="h-full rounded-full bg-indigo-300 flex items-center px-1.5"
          style={{ width: usedSpace }}
        />
      </div>
      <p className="text-sm font-semibold">
        {usedSpace} <span className="font-light">or</span>{" "}
        {formatBytes(drive.used)} <span className="font-light">used of</span>{" "}
        {formatBytes(drive.size)}
      </p>
    </div>
  );
}
