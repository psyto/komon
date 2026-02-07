"use client";

import { useState, useEffect } from "react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useWallet, useConnection } from "@solana/wallet-adapter-react";
import { WalletMultiButton } from "@solana/wallet-adapter-react-ui";
import { cn } from "@/lib/utils";
import { MapPin, Trophy, User, Plus, Home } from "lucide-react";
import { SovereignNavBadge } from "@/components/sovereign";
import { SovereignIdentity, fetchSovereignIdentity } from "@/lib/solana/sovereign";

const navItems = [
  { href: "/", label: "Home", icon: Home },
  { href: "/problems", label: "Problems", icon: MapPin },
  { href: "/leaderboard", label: "Leaderboard", icon: Trophy },
  { href: "/profile", label: "Profile", icon: User },
];

export function Navigation() {
  const pathname = usePathname();
  const { publicKey, connected } = useWallet();
  const { connection } = useConnection();
  const [sovereignIdentity, setSovereignIdentity] = useState<SovereignIdentity | null>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    async function loadIdentity() {
      if (!publicKey) {
        setSovereignIdentity(null);
        return;
      }

      setLoading(true);
      try {
        const identity = await fetchSovereignIdentity(connection, publicKey);
        setSovereignIdentity(identity);
      } catch (error) {
        console.error("Error fetching SOVEREIGN identity:", error);
        setSovereignIdentity(null);
      } finally {
        setLoading(false);
      }
    }

    loadIdentity();
  }, [publicKey, connection]);

  return (
    <header className="sticky top-0 z-50 w-full border-b bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
      <div className="container flex h-14 items-center">
        <div className="mr-4 flex">
          <Link href="/" className="mr-6 flex items-center space-x-2">
            <span className="font-bold text-xl">Komon</span>
          </Link>
          <nav className="flex items-center space-x-6 text-sm font-medium">
            {navItems.map((item) => {
              const Icon = item.icon;
              return (
                <Link
                  key={item.href}
                  href={item.href}
                  className={cn(
                    "flex items-center gap-2 transition-colors hover:text-foreground/80",
                    pathname === item.href ? "text-foreground" : "text-foreground/60"
                  )}
                >
                  <Icon className="h-4 w-4" />
                  {item.label}
                </Link>
              );
            })}
          </nav>
        </div>
        <div className="flex flex-1 items-center justify-end space-x-3">
          {connected && <SovereignNavBadge identity={sovereignIdentity} loading={loading} />}
          <WalletMultiButton className="!bg-primary !text-primary-foreground !rounded-md !h-9 !text-sm !font-medium hover:!bg-primary/90" />
          {connected && (
            <Link
              href="/problems/new"
              className="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"
            >
              <Plus className="h-4 w-4" />
              Post Problem
            </Link>
          )}
        </div>
      </div>
    </header>
  );
}
