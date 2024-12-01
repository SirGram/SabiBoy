import Layout from "../../components/Layout";
import { useOptions } from "../../context/OptionsContext";

export default function Options() {
  const { options, toggleShowFrame } = useOptions();

  return (
    <Layout>
      <div className="flex flex-col gap-4 h-full items-center py-20">
        <label className="flex items-center space-x-2">
          <span>Show Frame</span>
          <input
            type="checkbox"
            checked={options.showFrame}
            onChange={toggleShowFrame}
            className="size-6"
          />
        </label>
      </div>
    </Layout>
  );
}
