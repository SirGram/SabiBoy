import React from "react";
import "./style.css";
import { useOptions } from "../../../../context/OptionsContext";
// https://codepen.io/brundolf/pen/beagbQ

export function GameboyFrame({
  handleKeyDown,
  handleKeyUp,
  children,
}: {
  handleKeyDown: (event: KeyboardEvent) => void;
  handleKeyUp: (event: KeyboardEvent) => void;
  children: React.ReactNode;
}) {
  function handleGameboyKey(key: string, isKeyDown: boolean) {
    const event = new KeyboardEvent(isKeyDown ? "keydown" : "keyup", { key });
    if (isKeyDown) {
      handleKeyDown(event);
    } else {
      handleKeyUp(event);
    }
  }
  const { options } = useOptions();
  return !options.showFrame ? (
    <div className="p-2 bg-base-border rounded-md">
      <div style={{ width: "330px", height: "297px" }}>{children}</div>
    </div>
  ) : (
    <div className="gameboy" id="GameBoy">
      <div className="screen-area">
        <div className="display" id="mainCanvas">
          {children}
        </div>

        <div className="label">
          <div className="title mr-2">SABIBOY</div>
          <div className="subtitle">
            <span className="c">C</span>
            <span className="o1">O</span>
            <span className="l">L</span>
            <span className="o2">O</span>
            <span className="r">R</span>
          </div>
        </div>
      </div>

      <div className="nintendo">Noentiendo</div>

      <div className="controls">
        <div className="dpad">
          <div
            className="up"
            onMouseDown={() => handleGameboyKey("ArrowUp", true)}
            onMouseUp={() => handleGameboyKey("ArrowUp", false)}
          >
            <i className="fa fa-caret-up"></i>
          </div>
          <div
            className="right"
            onMouseDown={() => handleGameboyKey("ArrowRight", true)}
            onMouseUp={() => handleGameboyKey("ArrowRight", false)}
          >
            <i className="fa fa-caret-right"></i>
          </div>
          <div
            className="down"
            onMouseDown={() => handleGameboyKey("ArrowDown", true)}
            onMouseUp={() => handleGameboyKey("ArrowDown", false)}
          >
            <i className="fa fa-caret-down"></i>
          </div>
          <div
            className="left"
            onMouseDown={() => handleGameboyKey("ArrowLeft", true)}
            onMouseUp={() => handleGameboyKey("ArrowLeft", false)}
          >
            <i className="fa fa-caret-left"></i>
          </div>
          <div className="middle"></div>
        </div>
        <div className="a-b">
          <div
            className="b"
            onMouseDown={() => handleGameboyKey("z", true)}
            onMouseUp={() => handleGameboyKey("z", false)}
          >
            B
          </div>
          <div
            className="a"
            onMouseDown={() => handleGameboyKey("x", true)}
            onMouseUp={() => handleGameboyKey("x", false)}
          >
            A
          </div>
        </div>
      </div>
      <div className="start-select">
        <div
          className="select"
          onMouseDown={() => handleGameboyKey("Backspace", true)}
          onMouseUp={() => handleGameboyKey("Backspace", false)}
        >
          SELECT
        </div>
        <div
          className="start"
          onMouseDown={() => handleGameboyKey("Enter", true)}
          onMouseUp={() => handleGameboyKey("Enter", false)}
        >
          START
        </div>
      </div>

      <div className="speaker">
        {Array(64)
          .fill(null)
          .map((_, i) => (
            <div
              key={i}
              className={`dot ${
                i % 8 === 0 ? "placeholder" : i % 2 === 0 ? "open" : "closed"
              }`}
            ></div>
          ))}
      </div>
    </div>
  );
}
