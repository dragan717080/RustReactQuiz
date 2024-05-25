import { BsGithub, BsHouseFill } from "react-icons/bs";

export const Footer = () => {
  return (
    <footer className="text-gray-400 m-5 flex justify-center items-center gap-10 mt-auto">
      <a href='https://github.com/dragan717080'>
        <BsGithub className="h-7 w-7 text-gray-600 dark:text-gray-300 hover:!text-[#22C55E]" />
      </a>
      <a href='https://three-portfolio-seven.vercel.app/'>
        <BsHouseFill className="h-7 w-7 text-gray-600 dark:text-gray-300 hover:!text-[#22C55E]" />
      </a>
    </footer>
  );
};
