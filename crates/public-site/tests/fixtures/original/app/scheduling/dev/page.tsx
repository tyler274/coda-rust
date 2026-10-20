import type { Metadata } from "next";
import { pageMetadata } from "@/lib/metadata";
import UtilityPageShell from "@/components/UtilityPageShell";

const bookingUrl =
  "https://calendar.google.com/calendar/u/0/appointments/schedules/AcZssZ3g0UUNb-12RTQS50bhsuRrG2PmsMehtQ9wOGpbjVaGLEmrUp8prf1L6JGJGWYABkunI_4WnuDa";
const bookingEmbedUrl =
  "https://calendar.google.com/calendar/appointments/schedules/AcZssZ3g0UUNb-12RTQS50bhsuRrG2PmsMehtQ9wOGpbjVaGLEmrUp8prf1L6JGJGWYABkunI_4WnuDa";

export const metadata: Metadata = {
  // A personal booking link: shared directly, never indexed, with its own card.
  ...pageMetadata({
    title: "Schedule with Dev Patale",
    description:
      "Book a conversation with Dev Patale about clinical environments, health-agent evaluation, and health AI research.",
    path: "/scheduling/dev",
    image: { url: "/scheduling/dev/opengraph-image", width: 1200, height: 630, alt: "Schedule with Dev Patale at Coda Health" },
  }),
  robots: {
    index: false,
    follow: false,
  },
};

export default function DevSchedulingPage() {
  return (
    <UtilityPageShell>
      <main>
        <header className="hero">
          <div className="hero-copy">
            <p className="kicker">
              Research conversation / 30 minutes
            </p>
            <h1>
              Schedule time with Dev.
            </h1>
            <p className="lede">
              Book a 30-minute conversation about clinical environments, health-agent evaluation,
              or a research problem your team needs to make executable.
            </p>
          </div>
        </header>
        <section className="pb-20 pt-12 sm:pb-28 sm:pt-14">
          <div className="wrap">
            <div className="card overflow-hidden">
              <div className="flex min-h-14 flex-wrap items-center justify-between gap-3 border-b border-hairline bg-accent-bg px-4 py-3 sm:px-5">
                <div>
                  <p className="micro text-ink-light">
                    Google Calendar
                  </p>
                  <p className="mt-1 text-[0.84rem] font-semibold text-ink">30 min with Dev</p>
                </div>
                <span className="rounded-full border border-accent-border bg-surface px-3 py-1 micro text-accent">
                  Live availability
                </span>
              </div>
              <iframe
                src={bookingEmbedUrl}
                title="Schedule time with Dev Patale"
                className="h-[720px] w-full border-0 sm:h-[800px] lg:h-[760px]"
                loading="eager"
                referrerPolicy="strict-origin-when-cross-origin"
              />
            </div>

            <div className="mt-4 flex flex-col gap-2 text-sm text-ink-light sm:flex-row sm:items-center sm:justify-between">
              <p>Meetings are scheduled through Google Calendar.</p>
              <a className="ghost-link ghost-link--inline text-[0.68rem]" href={bookingUrl}>
                Open in Google Calendar
              </a>
            </div>
          </div>
        </section>
      </main>
    </UtilityPageShell>
  );
}
