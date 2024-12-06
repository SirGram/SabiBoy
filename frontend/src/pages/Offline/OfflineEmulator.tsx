import Layout from "../../components/Layout";
import { useGameboy } from "../../context/GameboyContext";
import Emulator from "../Game/Emulator";

export default function OfflineEmulator() {
  const {currentGame, setCurrentGame} = useGameboy();

  return (
    <Layout>

      <Emulator/>
      
    </Layout>
  )}