import { LogoIcon } from "./Icons";
import { ModeToggle } from "./mode-toggle";

export const Navbar = () => {

  return (
    <header className="sticky border-b-[1px] top-0 z-40 w-full bg-white dark:border-b-slate-700 dark:bg-background">
      <div className="m-auto">
        <div className="container h-14 px-4 w-screen flex justify-between items-center">
          <a
            rel="noreferrer noopener"
            href="/"
            className="ml-2 font-bold text-xl flex"
          >
            <LogoIcon />
            Quiz App by Dragan
          </a>
          <ModeToggle />
        </div>
      </div>
    </header>
  );
};
