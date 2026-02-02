import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { Navigation } from "@/components/ui/navigation";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Komon - Direct the Future",
  description: "A global protocol for civic problem-solving. Identify local problems, propose solutions, and get rewarded when they work.",
  keywords: ["civic", "problem-solving", "community", "blockchain", "solana"],
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <div className="min-h-screen flex flex-col">
          <Navigation />
          <main className="flex-1">
            {children}
          </main>
          <footer className="border-t py-6 px-4 text-center text-sm text-muted-foreground">
            <p>Komon - Shared problems deserve shared solutions</p>
          </footer>
        </div>
      </body>
    </html>
  );
}
