const LoadingScreen = () => {
  const bounceKeyframes = `
    @keyframes bounce {
      0% { transform: translateX(0); }
      50% { transform: translateX(192px); }
      100% { transform: translateX(0); }
    }
  `;

  return (
    <div className="flex flex-col items-center justify-center w-full h-screen bg-black">
      <div className="flex flex-col items-center space-y-6">
        {/* Enhanced title with text shadow and larger size */}
        <div className="mb-8">
          <h1
            className="text-primary font-bold text-4xl tracking-wider"
            style={{
              fontFamily: "system-ui",
              textShadow: `
                0 0 10px rgba(155, 188, 15, 0.3),
                0 0 20px rgba(155, 188, 15, 0.2)
              `,
            }}
          >
            SABIBOY
          </h1>
        </div>

        {/* Progress bar container */}
        <div className="w-64 h-5 bg-black border-2 border-primary p-0.5">
          {/* Bouncing loading bar */}
          <div
            className="h-full w-16 bg-primary"
            style={{
              animation: "bounce 2s linear infinite",
            }}
          />
        </div>

        <p className="text-primary font-mono text-lg mt-4">Loading...</p>

        <style>{bounceKeyframes}</style>
      </div>
    </div>
  );
};

export default LoadingScreen;
