import { ImageResponse } from "next/og";
import { siteOrigin } from "@/lib/site";

export const runtime = "edge";
export const alt = "Schedule with Dev Patale at Coda Health";
export const size = {
  width: 1200,
  height: 630,
};
export const contentType = "image/png";

export default function Image() {
  const logoUrl = `${siteOrigin}/brand/coda-health-logo-transparent.png`;

  return new ImageResponse(
    (
      <div
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          padding: 32,
          color: "#0a0a0a",
          background: "#ebece7",
          fontFamily: "Inter, Arial, sans-serif",
        }}
      >
        <div
          style={{
            width: "100%",
            height: "100%",
            display: "flex",
            flexDirection: "column",
            overflow: "hidden",
            border: "1px solid rgba(10,18,13,0.16)",
            borderRadius: 24,
            background: "#fbfbfa",
            boxShadow: "0 22px 70px rgba(22,35,25,0.08)",
          }}
        >
          <div
            style={{
              height: 76,
              display: "flex",
              alignItems: "center",
              justifyContent: "space-between",
              padding: "0 34px",
              borderBottom: "1px solid rgba(10,18,13,0.09)",
            }}
          >
            <img
              alt="Coda Health"
              src={logoUrl}
              style={{ width: 224, height: 45, objectFit: "contain" }}
            />
            <span
              style={{
                color: "#707570",
                fontSize: 14,
                fontWeight: 600,
                letterSpacing: 1.6,
                textTransform: "uppercase",
              }}
            >
              Calendar / Dev
            </span>
          </div>

          <div
            style={{
              flex: 1,
              display: "flex",
              alignItems: "stretch",
              padding: "46px 52px",
            }}
          >
            <div
              style={{
                flex: 1,
                display: "flex",
                flexDirection: "column",
                justifyContent: "center",
                paddingRight: 48,
              }}
            >
              <span
                style={{
                  color: "#155f43",
                  fontSize: 15,
                  fontWeight: 600,
                  letterSpacing: 1.8,
                  textTransform: "uppercase",
                }}
              >
                Research conversation / 30 minutes
              </span>
              <span
                style={{
                  marginTop: 24,
                  fontFamily: "Georgia, serif",
                  fontSize: 78,
                  lineHeight: 0.94,
                  letterSpacing: -3.5,
                }}
              >
                Schedule time with Dev.
              </span>
              <span
                style={{
                  maxWidth: 680,
                  marginTop: 25,
                  color: "#515451",
                  fontSize: 23,
                  lineHeight: 1.35,
                }}
              >
                Clinical environments, health-agent evaluation, and research problems worth a
                closer look.
              </span>
            </div>

            <div
              style={{
                width: 260,
                display: "flex",
                flexDirection: "column",
                justifyContent: "space-between",
                padding: 28,
                border: "1px solid rgba(21,95,67,0.18)",
                borderRadius: 16,
                background: "#edf3f0",
              }}
            >
              <span
                style={{
                  color: "#155f43",
                  fontSize: 13,
                  fontWeight: 600,
                  letterSpacing: 1.4,
                  textTransform: "uppercase",
                }}
              >
                Live availability
              </span>
              <div style={{ display: "flex", flexDirection: "column" }}>
                <span style={{ color: "#707570", fontSize: 16 }}>Google Calendar</span>
                <span
                  style={{
                    marginTop: 8,
                    fontFamily: "Georgia, serif",
                    fontSize: 34,
                    lineHeight: 1,
                  }}
                >
                  30 min with Dev
                </span>
              </div>
              <span style={{ color: "#707570", fontSize: 14 }}>codahealth.ai/scheduling/dev</span>
            </div>
          </div>
        </div>
      </div>
    ),
    size,
  );
}
