import ModeToggle from "@/components/mode_toggle/ModeToggle.tsx";
import YoutubeLogo from "@/assets/YoutubeLogo.tsx";
import NavMenu from "@/assets/NavMenu.tsx";
import Search from "@/components/navigation/Search.tsx";
import { Avatar, AvatarFallback } from "@/components/ui/avatar.tsx";
import { useCallback, useEffect, useState } from "react";
import { useAuth0 } from "@auth0/auth0-react";

const Navigation = () => {
  const [searchText, setSearchText] = useState("");

  const { loginWithRedirect, getAccessTokenSilently } = useAuth0();
  const getToken = useCallback(async () => {
    const token = await getAccessTokenSilently();
    console.log("Token ", token);
  }, [getAccessTokenSilently]);

  useEffect(() => {
    getToken();
  }, []);

  return (
    <nav className="app-nav flex">
      <div className="w-full h-full flex items-center justify-between mx-6">
        <div className="flex items-center justify-center gap-6">
          <NavMenu className="cursor-pointer" />
          <YoutubeLogo className="cursor-pointer" />
        </div>
        <div>
          <Search
            value={searchText}
            updateValue={(e) => setSearchText(e?.target?.value)}
            handleSearch={() => {
              console.log("Search text", searchText);
            }}
          />
        </div>
        <div className="flex items-center justify-center gap-6">
          <Avatar
            className="cursor-pointer"
            onClick={() => {
              loginWithRedirect();
            }}
          >
            <AvatarFallback>U</AvatarFallback>
          </Avatar>
          <ModeToggle />
        </div>
      </div>
    </nav>
  );
};

export default Navigation;
