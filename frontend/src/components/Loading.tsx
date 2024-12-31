import { useState, useEffect } from "react";

const Loading = () => {
  const directions = ["up", "right", "down", "left"];
  const [step, setStep] = useState(0);
  
  useEffect(() => {
    const interval = setInterval(() => {
      setStep(prev => (prev + 1) % (directions.length * 2));
    }, 100);

    return () => clearInterval(interval);
  }, []);

  const activeSegments = directions.filter((_, index) => {
    if (step < directions.length) {
      // Illumination phase (0-3)
      return index <= step;
    } else {
      // Deillumination phase (4-7)
      return index > (step - directions.length);
    }
  });

  return (
    <div className="relative w-24 h-24">
      {/* Center circle */}
      <div className="absolute top-8 left-8 w-8 h-8 bg-gray-800">
        <div className="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-4 h-4 rounded-full bg-base-background" />
      </div>

      {/* Up button */}
      <div className="absolute top-0 left-8 right-8 h-8 rounded-t-lg overflow-hidden bg-gray-800">
        <div
          className={`absolute inset-0 bg-gradient-to-t from-gray-800 to-primary transition-opacity duration-200 ease-out ${
            activeSegments.includes("up") ? "opacity-100" : "opacity-0"
          }`}
        />
      </div>

      {/* Right button */}
      <div className="absolute top-8 right-0 bottom-8 w-8 rounded-r-lg overflow-hidden bg-gray-800">
        <div
          className={`absolute inset-0 bg-gradient-to-r from-gray-800 to-primary transition-opacity duration-200 ease-out ${
            activeSegments.includes("right") ? "opacity-100" : "opacity-0"
          }`}
        />
      </div>

      {/* Down button */}
      <div className="absolute bottom-0 left-8 right-8 h-8 rounded-b-lg overflow-hidden bg-gray-800">
        <div
          className={`absolute inset-0 bg-gradient-to-b from-gray-800 to-primary transition-opacity duration-200 ease-out ${
            activeSegments.includes("down") ? "opacity-100" : "opacity-0"
          }`}
        />
      </div>

      {/* Left button */}
      <div className="absolute top-8 left-0 bottom-8 w-8 rounded-l-lg overflow-hidden bg-gray-800">
        <div
          className={`absolute inset-0 bg-gradient-to-l from-gray-800 to-primary transition-opacity duration-200 ease-out ${
            activeSegments.includes("left") ? "opacity-100" : "opacity-0"
          }`}
        />
      </div>

      {/* Inner highlights */}
      <div className="absolute top-1 left-9 right-9 h-1 bg-gray-700 rounded-t-lg" />
      <div className="absolute top-9 right-1 bottom-9 w-1 bg-gray-700 rounded-r-lg" />
      <div className="absolute bottom-1 left-9 right-9 h-1 bg-gray-700 rounded-b-lg" />
      <div className="absolute top-9 left-1 bottom-9 w-1 bg-gray-700 rounded-l-lg" />
    </div>
  );
};

export default Loading;