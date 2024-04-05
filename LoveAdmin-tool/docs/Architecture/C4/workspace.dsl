workspace "LoveAdmin Reconcilliation tool" {

    model {
        user = person "User" "Club admin"
        LoveAdminTool = softwareSystem "loveadmin_tool" "Love Admin Reconcilliation Tool"
        FA_Wholegame = softwareSystem "FA Wholegame"
        LoveAdmin = softwareSystem "LoveAdmin"

        user -> LoveAdminTool "Uses"
        user -> FA_Wholegame "Download xlsx"
        user -> LoveAdmin "Download csv"
    }

    views {
        systemContext LoveAdminTool "loveadmin_tool" "SystemContext" {
            include *
            autoLayout
        }
        
        
        styles {
            element "LoveAdminToolm" {
                background #1168bd
                color #ffffff
            }
            element "Person" {
                shape person
                background #08427b
                color #ffffff
            }
        }
    }

}
