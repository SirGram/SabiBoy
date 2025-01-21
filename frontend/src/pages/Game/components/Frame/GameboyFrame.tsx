// https://codepen.io/brundolf/pen/beagbQ

import React from "react";
import "./style.css";
import { useOptions } from "../../../../context/OptionsContext";

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
  function handleTouchStart(key: string) {
    handleGameboyKey(key, true);
  }

  function handleTouchEnd(key: string) {
    handleGameboyKey(key, false);
  }
  const { options } = useOptions();
  return !options.showFrame ? (
    <div className="p-2 bg-base-border rounded-md ">
     {children}
    </div>
  ) : (
    <div className="gameboy" id="GameBoy">
      <div className="screen-area">
        <div className="display" id="mainCanvas">
          {children}
        </div>

        <div className="label">
          <div className="title mr-2">SABIBOY</div>
          <div className="subtitle"> {/* TODO: show only on CGB */}
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
            onTouchStart={() => handleTouchStart("ArrowUp")}
            onTouchEnd={() => handleTouchEnd("ArrowUp")}
          >
            <i className="fa fa-caret-up"></i>
          </div>
          <div
            className="right"
            onMouseDown={() => handleGameboyKey("ArrowRight", true)}
            onMouseUp={() => handleGameboyKey("ArrowRight", false)}
            onTouchStart={() => handleTouchStart("ArrowRight")}
            onTouchEnd={() => handleTouchEnd("ArrowRight")}
          >
            <i className="fa fa-caret-right"></i>
          </div>
          <div
            className="down"
            onMouseDown={() => handleGameboyKey("ArrowDown", true)}
            onMouseUp={() => handleGameboyKey("ArrowDown", false)}
            onTouchStart={() => handleTouchStart("ArrowDown")}
            onTouchEnd={() => handleTouchEnd("ArrowDown")}
          >
            <i className="fa fa-caret-down"></i>
          </div>
          <div
            className="left"
            onMouseDown={() => handleGameboyKey("ArrowLeft", true)}
            onMouseUp={() => handleGameboyKey("ArrowLeft", false)}
            onTouchStart={() => handleTouchStart("ArrowLeft")}
            onTouchEnd={() => handleTouchEnd("ArrowLeft")}
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
            onTouchStart={() => handleTouchStart("z")}
            onTouchEnd={() => handleTouchEnd("z")}
          >
            B
          </div>
          <div
            className="a"
            onMouseDown={() => handleGameboyKey("x", true)}
            onMouseUp={() => handleGameboyKey("x", false)}
            onTouchStart={() => handleTouchStart("x")}
            onTouchEnd={() => handleTouchEnd("x")}
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
          onTouchStart={() => handleTouchStart("Backspace")}
          onTouchEnd={() => handleTouchEnd("Backspace")}
        >
          SELECT
        </div>
        <div
          className="start"
          onMouseDown={() => handleGameboyKey("Enter", true)}
          onMouseUp={() => handleGameboyKey("Enter", false)}
          onTouchStart={() => handleTouchStart("Enter")}
          onTouchEnd={() => handleTouchEnd("Enter")}
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
