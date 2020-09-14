import React from "react";
import { Link } from "react-router-dom";
import { Row, Col, Card, Avatar, Typography, Space, Rate } from "antd";
import { DownloadOutlined } from "@ant-design/icons";
import SearchComponent from "./search";

// import { gql } from "@apollo/client";

const { Text, Title } = Typography;

// const APPS = gql`
//   query {
//     books {
//       id
//       pkgname
//       icons {
//         type
//         name
//       }
//     }
//   }
// `;

const data = [
  {
    name: "Visual Studio Code",
    icon:
      "https://upload.wikimedia.org/wikipedia/commons/2/2d/Visual_Studio_Code_1.18_icon.svg",
    rate: 4.5,
    owner: "Microsoft",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/4/44/Microsoft_logo.svg",
    downloadCount: "10M",
    link: "/apps/visual-studio-code",
  },
  {
    name: "Atom",
    icon: "https://upload.wikimedia.org/wikipedia/commons/7/7b/Icon_Atom.svg",
    rate: 4.6,
    owner: "Github",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/a/ae/Github-desktop-logo-symbol.svg",
    downloadCount: "8M",
    link: "/apps/atom",
  },
  {
    name: "Android Studio",
    icon:
      "https://upload.wikimedia.org/wikipedia/commons/3/34/Android_Studio_icon.svg",
    rate: 4.6,
    owner: "Google",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/5/53/Google_%22G%22_Logo.svg",
    downloadCount: "8M",
    link: "/apps/android-studio",
  },
  {
    name: "Arduino IDE",
    icon:
      "https://upload.wikimedia.org/wikipedia/commons/8/87/Arduino_Logo.svg",
    rate: 4.6,
    owner: "Arduino",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/8/87/Arduino_Logo.svg",
    downloadCount: "8M",
    link: "/apps/android-studio",
  },
  {
    name: "Brackets",
    icon:
      "https://upload.wikimedia.org/wikipedia/commons/4/4c/Brackets_Icon.svg",
    rate: 4.6,
    owner: "Brackets",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/4/4c/Brackets_Icon.svg",
    downloadCount: "8M",
    link: "/apps/brackets",
  },
  {
    name: "IntelliJ IDEA",
    icon:
      "https://upload.wikimedia.org/wikipedia/commons/d/d5/IntelliJ_IDEA_Logo.svg",
    rate: 4.6,
    owner: "IntelliJ IDEA",
    owner_logo:
      "https://upload.wikimedia.org/wikipedia/commons/d/d5/IntelliJ_IDEA_Logo.svg",
    downloadCount: "8M",
    link: "/apps/intellij-idea",
  },
  {
    name: "Eclipse IDE",
    icon:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.eclipse.Committers.png",
    rate: 4.6,
    owner: "Eclipse Foundation",
    owner_logo:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.eclipse.Committers.png",
    downloadCount: "8M",
    link: "/apps/eclipse-ide",
  },
  {
    name: "Blender",
    icon:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.blender.Blender.png",
    rate: 4.6,
    owner: "Blender",
    owner_logo:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.blender.Blender.png",
    downloadCount: "8M",
    link: "/apps/blender",
  },
  {
    name: "Inkscape",
    icon:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.inkscape.Inkscape.png",
    rate: 4.6,
    owner: "Inkscape",
    owner_logo:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.inkscape.Inkscape.png",
    downloadCount: "8M",
    link: "/apps/inkscape",
  },
  {
    name: "GIMP",
    icon:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.gimp.GIMP.png",
    rate: 4.6,
    owner: "Gimp",
    owner_logo:
      "https://flathub.org/repo/appstream/x86_64/icons/128x128/org.gimp.GIMP.png",
    downloadCount: "8M",
    link: "/apps/gimp",
  },
];

const Home = () => {
  return (
    <>
      <SearchComponent />
      <Row gutter={[20, 20]}>
        {data.map((app) => (
          <Col span={4}>
            <Link to={app.link}>
              <Card hoverable>
                <center style={{ fontSize: "12px" }}>
                  <div
                    style={{
                      width: "84px",
                      height: "84px",
                      backgroundImage: `url("${app.icon}")`,
                      backgroundSize: "contain",
                      backgroundRepeat: "no-repeat",
                      backgroundPosition: "center",
                      margin: "12px",
                    }}
                  ></div>
                  <Title style={{ fontSize: "14px", margin: "25px 0 0 0" }}>
                    {app.name}
                  </Title>
                  <Col style={{ margin: 0 }}>
                    <Rate allowHalf defaultValue={app.rate} />
                  </Col>
                </center>

                <Row
                  gutter={[0, 0]}
                  style={{
                    // padding: "20px 0px",
                    height: "30px",
                    lineHeight: "30px",
                  }}
                >
                  <Col
                    span={16}
                    style={{ textAlign: "left", overflow: "hidden" }}
                  >
                    <Space>
                      <Avatar
                        src={app.owner_logo}
                        shape="square"
                        size="small"
                      />
                      <Text style={{ fontSize: "12px" }}>{app.owner}</Text>
                    </Space>
                  </Col>

                  <Col span={8} style={{ textAlign: "right" }}>
                    <Space>
                      <Text style={{ fontSize: "12px" }}>
                        {app.downloadCount}
                      </Text>
                      <DownloadOutlined />
                    </Space>
                  </Col>
                </Row>
              </Card>
            </Link>
          </Col>
        ))}
      </Row>
    </>
  );
};

export default Home;
