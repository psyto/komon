import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatCurrency(amount: number | bigint): string {
  const num = typeof amount === "bigint" ? Number(amount) / 1_000_000 : amount;
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(num);
}

export function formatNumber(num: number): string {
  return new Intl.NumberFormat("en-US").format(num);
}

export function formatPercent(value: number, decimals: number = 1): string {
  return `${value.toFixed(decimals)}%`;
}

export function formatDate(date: Date | string | number): string {
  const d = new Date(date);
  return d.toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}

export function formatRelativeTime(date: Date | string | number): string {
  const d = new Date(date);
  const now = new Date();
  const diffMs = now.getTime() - d.getTime();
  const diffSec = Math.floor(diffMs / 1000);
  const diffMin = Math.floor(diffSec / 60);
  const diffHour = Math.floor(diffMin / 60);
  const diffDay = Math.floor(diffHour / 24);

  if (diffSec < 60) return "just now";
  if (diffMin < 60) return `${diffMin}m ago`;
  if (diffHour < 24) return `${diffHour}h ago`;
  if (diffDay < 7) return `${diffDay}d ago`;
  return formatDate(d);
}

export function truncateAddress(address: string, chars: number = 4): string {
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}

export function scaleLocation(lat: number, lng: number): { lat: bigint; lng: bigint } {
  return {
    lat: BigInt(Math.round(lat * 1_000_000)),
    lng: BigInt(Math.round(lng * 1_000_000)),
  };
}

export function unscaleLocation(lat: bigint | number, lng: bigint | number): { lat: number; lng: number } {
  return {
    lat: Number(lat) / 1_000_000,
    lng: Number(lng) / 1_000_000,
  };
}
