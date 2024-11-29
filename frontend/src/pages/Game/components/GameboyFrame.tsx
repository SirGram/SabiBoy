import React from "react";

export function GameboyFrame({ children }: { children: React.ReactNode }) {
  return (
    <div className="flex items-center justify-center  ">
      {/* Gameboy Container */}
      <div className="relative  w-48 h-80 rounded-md rounded-br-3xl shadow-lg border-2 border-base-border">
        {/* Screen */}
        <div className="absolute top-1 left-2 right-2 h-36  rounded-md flex items-center justify-center">
          {/* Dynamic Screen Content */}
          <div className="w-full h-full  rounded-sm overflow-hidden">
            {children}
          </div>
        </div>

        {/* Directional Pad */}
        <div className="absolute bottom-20 left-6 grid grid-cols-3 border-base-border">
          {/* Row 1 */}
          <div className="w-5 h-5 bg-transparent border-r-2 border-b-2 rounded-br-md  border-base-border"></div>{" "}
          {/* Top-left (empty space) */}
          <div className="w-5 h-5  shadow-md hover:bg-gray-400 flex items-center justify-center border-t-2  border-base-border">
            {/* Up button */}
          </div>
          <div className="w-5 h-5 bg-transparent border-l-2 rounded-bl-md border-b-2  border-base-border"></div>{" "}
          {/* Top-right (empty space) */}
          {/* Row 2 */}
          <div className="w-5 h-5 border-l-2 shadow-md hover:bg-gray-400 flex items-center justify-center  border-base-border">
            {/* Left button */}
          </div>
          <div className="w-5 h-5 bg-transparent"></div>{" "}
          {/* Center (empty space) */}
          <div className="w-5 h-5 border-r-2 shadow-md hover:bg-gray-400 flex items-center justify-center  border-base-border">
            {/* Right button */}
          </div>
          {/* Row 3 */}
          <div className="w-5 h-5 bg-transparent border-t-2 border-r-2 rounded-tr-md  border-base-border"></div>{" "}
          {/* Bottom-left (empty space) */}
          <div className="w-5 h-5 border-b-2  shadow-md hover:bg-gray-400 flex items-center justify-center  border-base-border">
            {/* Down button */}
          </div>
          <div className="w-5 h-5 bg-transparent border-t-2 border-l-2 rounded-tl-md  border-base-border"></div>{" "}
          {/* Bottom-right (empty space) */}
        </div>

        {/* A and B Buttons */}
        <div className="absolute bottom-20 right-4 flex gap-3 ">
          <div className="w-8 h-8 border-2 border-base-border rounded-full shadow-md my-2">
            {" "}
            <span className="text-sm font-bold text-black w-full h-full flex items-center justify-center">
              A
            </span>
          </div>
          <div className="w-8 h-8 border-2 border-base-border rounded-full shadow-md">
            {" "}
            <span className="text-sm font-bold text-black w-full h-full flex items-center justify-center">
              B
            </span>
          </div>
        </div>

        {/* Start and Select Buttons */}
        <div className="absolute bottom-10 left-1/2 transform -translate-x-1/2 flex gap-4">
          <div className="w-10 h-4 border-2 border-base-border rounded shadow-md"></div>
          <div className="w-10 h-4 border-2 border-base-border rounded shadow-md"></div>
        </div>

        {/* Speaker Dots */}
        <div className="absolute bottom-3 right-3 grid grid-cols-3 gap-[2px]">
          <div className="w-2 h-2 bg-muted/20 rounded-full"></div>
          <div className="w-2 h-2 bg-muted/20  rounded-full"></div>
          <div className="w-2 h-2 bg-muted/20  rounded-full"></div>
          <div className="w-2 h-2 bg-muted/20  rounded-full"></div>
          <div className="w-2 h-2 bg-muted/20  rounded-full"></div>
          <div className="w-2 h-2 bg-muted/20  rounded-full"></div>
        </div>
      </div>
    </div>
  );
}
