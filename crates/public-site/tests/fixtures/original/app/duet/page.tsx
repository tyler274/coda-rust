import type { CSSProperties } from "react";
import type { Metadata } from "next";
import Script from "next/script";
import SiteFooter from "@/components/SiteFooter";
import SiteNav from "@/components/SiteNav";
import { homepageHtml } from "@/lib/homepage-html.mjs";
import { pageMetadata } from "@/lib/metadata";
// Built on the homepage's system: the shared chrome, the homepage styles and
// script, and the homepage's Duet phones. Like the homepage, it does not load Tailwind.
import "../chrome.css";
import "../_homepage/site.css";

export const metadata: Metadata = pageMetadata({
  title: "Duet",
  description:
    "Duet helps you bring your health information together, notice what matters, and prepare for conversations with your doctor.",
  path: "/duet",
  image: {
    url: "/brand/duet/social-preview-v1.png",
    width: 1200,
    height: 630,
    alt: "Duet by Coda Health. A clearer picture of you.",
  },
});

const principles = [
  {
    title: "Bring your health together",
    body: "Connect Apple Health and one or more Epic MyChart organizations when you choose. Duet preserves where information came from and never writes back to those sources.",
  },
  {
    title: "Notice the few things that matter",
    body: "If you allow AI, Duet can show changes and patterns from your health information, with the source available for you to review.",
  },
  {
    title: "Carry the useful parts into care",
    body: "Save an insight, add what you want to discuss, review the prepared summary, and share it through the iPhone share sheet when you are ready.",
  },
];

const delay = (seconds: number) => ({ "--d": `${seconds}s` }) as CSSProperties;

export default function DuetPage() {
  return (
    <>
      <SiteNav />
      <main>
        <header className="hero">
          <div className="hero-copy">
            <div className="duet-brand duet-brand-hero rv">
              <span className="duet-brand-lockup">
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <img className="duet-brand-mark" src="/brand/duet/mark.svg" width="400" height="400" alt="" />
                {/* eslint-disable-next-line @next/next/no-img-element */}
                <img className="duet-brand-wordmark" src="/brand/duet/wordmark.svg" width="440" height="170" alt="Duet" />
              </span>
            </div>
            <p className="kicker rv" style={delay(0.03)}>Your health, in context</p>
            <h1 className="rv" style={delay(0.06)}>A clearer picture of you.</h1>
            <p className="lede rv" style={delay(0.12)}>
              Duet brings together the health information you choose, notices a
              few supported patterns, and helps you prepare the useful parts for
              a conversation with your doctor.
            </p>
            <p className="hero-meta micro rv" style={delay(0.18)}>
              iPhone · U.S. adults 18+ · Free · No advertising
            </p>
          </div>
        </header>

        <section className="sec sec-tight">
          <div className="wrap" dangerouslySetInnerHTML={{ __html: homepageHtml("duet-phones.html") }} />
        </section>

        <section className="sec">
          <div className="wrap tiles">
            {principles.map((principle, index) => (
              <div key={principle.title} className="tile rv" style={delay(index * 0.06)}>
                <h3>{principle.title}</h3>
                <p>{principle.body}</p>
              </div>
            ))}
          </div>
        </section>

        <section className="sec sec-close">
          <div className="wrap duet-note">
            <p>
              Duet is not emergency care and does not diagnose, prescribe, or
              replace your healthcare professional.
            </p>
          </div>
        </section>
      </main>
      <SiteFooter />
      <Script src="/homepage/site.js" strategy="afterInteractive" />
    </>
  );
}
