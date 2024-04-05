workspace "LoveAdmin Reconcilliation tool" {

    model {
        user = person "User" "Club admin"
        LoveAdminTool = softwareSystem "loveadmin_tool" "Love Admin Reconcilliation Tool" {
            LoveAdminTool-database = container "SQLite database"{
                tags "Database"
            }
            LoveAdminTool-database-if = container "db interface"
            LoveAdminTool-GUI = container "GUI Frontend"
            LoveAdminTool-xlsx_parser = container "Parse xlsx"
            LoveAdminTool-csv_parser = container "Parse csv"
            LoveAdminTool-db_query = container "Query db"
        }
        FA_Wholegame = softwareSystem "FA Wholegame"
        LoveAdmin = softwareSystem "LoveAdmin"

        user -> LoveAdminTool-GUI "Uses"
        user -> FA_Wholegame "Download xlsx"
        user -> LoveAdmin "Download csv"
        
        LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser
        LoveAdminTool-GUI -> LoveAdminTool-csv_parser
        LoveAdminTool-GUI -> LoveAdminTool-db_query
        LoveAdminTool-xlsx_parser ->  LoveAdminTool-database-if
        LoveAdminTool-csv_parser ->  LoveAdminTool-database-if
        LoveAdminTool-db_query ->  LoveAdminTool-database-if
        LoveAdminTool-database-if -> LoveAdminTool-database
    }

    views {
        systemLandscape LoveAdminTool_systemLandscape {
            include *
            autolayout
        }
    
        systemContext LoveAdminTool "loveadmin_tool" "SystemContext" {
            include * FA_Wholegame LoveAdmin
            autoLayout
        }
        
        container LoveAdminTool "Containers_All" {
            include *
            autolayout
        }
        
        dynamic LoveAdminTool "SimpleJourney" {
            title "Simple reconcile"
            autolayout
            user -> FA_Wholegame "Download xlsx info from FA Wholegame"
            user -> LoveAdmin "Download csv infor from LoveAdmin"
            user -> LoveAdminTool-GUI "load FA Wholegame xlsx into tool"
            user -> LoveAdminTool-GUI "load LoveAdmin csv into tool"
            user -> LoveAdminTool-GUI "Request Reconcilliation report"
        }
                
        dynamic LoveAdminTool "LoadFile" {
            title "Reconcile"
            autolayout
            user -> FA_Wholegame "Download xlsx info from FA Wholegame"
            user -> LoveAdmin "Download csv infor from LoveAdmin"
            user -> LoveAdminTool-GUI "load FA Wholegame xlsx into tool"
            LoveAdminTool-GUI -> LoveAdminTool-xlsx_parser "Send file to parser"
            LoveAdminTool-xlsx_parser -> LoveAdminTool-database-if "sennd parsed data to db if"
            LoveAdminTool-database-if -> LoveAdminTool-database "store data in db"
            user -> LoveAdminTool-GUI "load LoveAdmin csv into tool"
            LoveAdminTool-GUI -> LoveAdminTool-csv_parser "Send file to parser"
            LoveAdminTool-csv_parser -> LoveAdminTool-database-if "sennd parsed data to db if"
            LoveAdminTool-database-if -> LoveAdminTool-database "store data in db"
            user -> LoveAdminTool-GUI "Request Reconcilliation report"
            LoveAdminTool-GUI -> LoveAdminTool-db_query "Request query of data"
            LoveAdminTool-db_query -> LoveAdminTool-database-if "send query to db"
            LoveAdminTool-database-if -> LoveAdminTool-database "query data in db"
        }
        
        styles {
            element "Software System" {
                background #e1a6f5
                color #ffffff
            }
            element "Person" {
                shape person
                background #08427b
                color #ffffff
            }
            element "Database" {
                shape Cylinder
            }
        }
    }

}
