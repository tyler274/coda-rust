import { contactEmail } from "@/lib/site";
type LegalSection = {
  title: string;
  body: string[];
};

export const lastUpdated = "September 13, 2026";

export const termsSections: LegalSection[] = [
  {
    title: "Use of the site and services",
    body: [
      "Coda Health provides information about its products and services for physicians and patients, including Coda and the Duet app, along with ways to contact us, schedule conversations, and explore partnerships. You may use this website and any Coda Health services only for lawful purposes and in accordance with these Terms.",
      "You may not interfere with the security, availability, or proper operation of the site or services, attempt to access systems or data without authorization, or use the site or services in a way that violates applicable law or third-party rights.",
    ],
  },
  {
    title: "Website information",
    body: [
      "The content on this website is informational. It is not medical advice, diagnosis, treatment, or emergency care, and it is not a substitute for the judgment of a licensed healthcare professional. If you think you may have a medical emergency, call your local emergency number.",
      "Patient scenarios, examples, and product demonstrations shown on the site are synthetic and illustrative. They do not describe real patients or real clinical decisions.",
    ],
  },
  {
    title: "Coda for physicians",
    body: [
      "Coda is designed for licensed healthcare professionals making non-emergency, non-time-critical decisions. It organizes authorized patient information and presents assessments, options, or recommendations for the professional's independent review. It does not replace professional judgment or independently diagnose, prescribe, order, communicate, or carry out care.",
      "Coda is designed to keep the basis for an output reviewable, including the patient information used, source and timing information, material knowns and unknowns, and relevant limitations. The licensed healthcare professional makes every final clinical decision and approves any resulting action.",
      "Coda's clinical use, review model, information sources, authority boundaries, and limitations are described at codahealth.ai/clinical-transparency. A specific pilot or deployment may have additional written terms, labeling, and clinical or technical controls.",
    ],
  },
  {
    title: "Duet for patients",
    body: [
      "Duet is a separate patient-facing product that helps people organize health information they choose to connect, notice supported observations, and prepare information for conversations with a healthcare professional. Duet does not provide a diagnosis, personalized risk or prognosis, medical significance, urgency, treatment, testing direction, medication effect, or reassurance.",
      "Duet is not emergency care and does not replace a healthcare professional. Its own Terms of Service and Privacy Policy govern use of Duet.",
    ],
  },
  {
    title: "Accounts and communications",
    body: [
      "Some services may require account access, authenticated integrations, or authorization by a Coda Health representative. You are responsible for maintaining the confidentiality of credentials and for activity that occurs through authorized access you provide.",
      "When you contact us or schedule time with us, you agree to provide accurate information and to receive communications related to your request. You can opt out of non-essential communications at any time.",
    ],
  },
  {
    title: "Product-specific terms",
    body: [
      "The Duet app is governed by its own Terms of Service, available at codahealth.ai/duet/terms. Its Privacy Policy is available at codahealth.ai/duet/privacy. Pilots, deployments, research collaborations, and partnerships are governed by separate written agreements, which control where they conflict with these Terms.",
    ],
  },
  {
    title: "Intellectual property",
    body: [
      "The site, trademarks, logos, text, graphics, software, and other content are owned by Coda Health or its licensors and are protected by intellectual-property laws. Except as expressly permitted, you may not copy, modify, distribute, sell, or create derivative works from Coda Health materials.",
      "If you provide feedback or suggestions, Coda Health may use them without restriction or obligation to you.",
    ],
  },
  {
    title: "Third-party services",
    body: [
      "The site and services may link to or integrate with third-party platforms, such as scheduling and calendar tools. Coda Health is not responsible for third-party services, content, policies, or practices. Your use of third-party services may be governed by their own terms and policies.",
    ],
  },
  {
    title: "Disclaimers",
    body: [
      "The site and services are provided on an \"as is\" and \"as available\" basis. To the fullest extent permitted by law, Coda Health disclaims all warranties, whether express, implied, statutory, or otherwise, including warranties of merchantability, fitness for a particular purpose, and non-infringement.",
    ],
  },
  {
    title: "Limitation of liability",
    body: [
      "To the fullest extent permitted by law, Coda Health will not be liable for indirect, incidental, special, consequential, exemplary, or punitive damages, or for lost profits, lost data, or business interruption, arising from or related to your use of the site or services.",
    ],
  },
  {
    title: "Changes",
    body: [
      "Coda Health may update these Terms from time to time. The updated version will be posted on this page with a new effective date. Continued use of the site or services after changes become effective means you accept the updated Terms.",
    ],
  },
  {
    title: "Contact",
    body: [
      `Questions about these Terms may be sent to ${contactEmail}.`,
    ],
  },
];

export const privacySections: LegalSection[] = [
  {
    title: "Information we collect",
    body: [
      "We collect information you provide directly, such as name, email address, organization, role, and the details of your inquiry, when you contact us, schedule a conversation, or otherwise correspond with us.",
      "We also collect basic technical information such as device, browser, log, referral, and usage information through hosting, analytics, security, and performance tools, some of which may use cookies or similar technologies.",
    ],
  },
  {
    title: "How we use information",
    body: [
      "We use information to operate, maintain, secure, and improve the site and services; respond to inquiries; schedule and manage conversations; provide support; develop and evaluate Coda Health products; comply with legal obligations; and protect Coda Health, users, and third parties.",
    ],
  },
  {
    title: "How we share information",
    body: [
      "We share information with service providers that help us host, operate, secure, analyze, or support the site and services, such as hosting, scheduling, email, and analytics providers. These providers are authorized to use information only as needed to provide services to us.",
      "We may also share information when required by law, legal process, or governmental request; to protect rights, safety, and security; in connection with a merger, financing, acquisition, or similar corporate transaction; or with your direction or consent.",
      "We do not sell personal information, and we do not use it for third-party advertising.",
    ],
  },
  {
    title: "Health information",
    body: [
      "This website does not ask you to submit medical records or personal health information, and the patient examples shown in product demonstrations are synthetic. Please do not include personal health information in inquiries sent through the site.",
      "The Duet app is governed by its own Privacy Policy, available at codahealth.ai/duet/privacy, which controls for information handled in the app.",
      "Where Coda Health processes clinical, de-identified, or operational data for a healthcare organization, that processing is governed by separate written agreements, such as a business associate agreement or data processing addendum, which control where they conflict with this Policy.",
    ],
  },
  {
    title: "Retention and security",
    body: [
      "We retain information for as long as reasonably necessary to provide the site and services, satisfy the purposes described in this Policy, comply with legal obligations, resolve disputes, and enforce agreements.",
      "We use reasonable administrative, technical, and organizational safeguards designed to protect information. No method of transmission or storage is completely secure.",
    ],
  },
  {
    title: "Your choices",
    body: [
      "You may contact us to request access, correction, deletion, or other handling of your information, subject to applicable law and verification. You may also unsubscribe from non-transactional communications where an unsubscribe option is provided.",
      "Depending on where you live, you may have additional rights under applicable privacy laws. We honor such requests as required by the law that applies to you.",
    ],
  },
  {
    title: "Children",
    body: [
      "The site is not directed to children under 13, and we do not knowingly collect personal information from children. If you believe a child has provided us personal information, contact us and we will delete it.",
    ],
  },
  {
    title: "Changes",
    body: [
      "We may update this Privacy Policy from time to time. The updated version will be posted on this page with a new effective date.",
    ],
  },
  {
    title: "Contact",
    body: [
      `Questions about this Privacy Policy may be sent to ${contactEmail}.`,
    ],
  },
];
